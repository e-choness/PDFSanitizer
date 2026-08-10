#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod pdf_sanitizer;
mod settings;

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tauri::{State, Manager};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SanitizationSettings {
    pub remove_metadata: bool,
    pub remove_scripts: bool,
    pub remove_embedded_files: bool,
    pub compress_images: bool,
    pub high_compression: bool,
    pub strip_external_links: bool,
    pub font_subsetting: bool,
    pub max_concurrent: u32,
    pub output_folder: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileToProcess {
    pub id: f64,
    pub path: String,
}

pub struct AppState {
    settings: Arc<Mutex<SanitizationSettings>>,
}

#[tauri::command]
fn load_settings(state: State<AppState>) -> SanitizationSettings {
    let settings = state.settings.lock().unwrap();
    settings.clone()
}

#[tauri::command]
fn save_settings(settings: SanitizationSettings, state: State<AppState>) {
    let mut app_settings = state.settings.lock().unwrap();
    *app_settings = settings.clone();
    settings::save_settings(&settings).ok();
}

#[tauri::command]
async fn select_folder() -> Option<String> {
    // Folder selection is handled by the frontend via Tauri's native dialog API
    // This command can be removed once frontend handles it directly
    None
}

#[tauri::command]
async fn process_files(
    files: Vec<FileToProcess>,
    settings: SanitizationSettings,
    app_handle: tauri::AppHandle,
    _state: State<'_, AppState>,
) -> Result<String, String> {
    let settings = Arc::new(settings);
    let max_concurrent = settings.max_concurrent as usize;

    tokio::spawn(async move {
        let semaphore = Arc::new(tokio::sync::Semaphore::new(max_concurrent));

        let mut handles = vec![];

        for file in files {
            let permit = semaphore.clone().acquire_owned().await.unwrap();
            let settings = Arc::clone(&settings);
            let app_handle = app_handle.clone();
            let file_id = file.id;
            let file_path = file.path.clone();

            let handle = tokio::spawn(async move {
                let _permit = permit;

                match pdf_sanitizer::sanitize_pdf(&file_path, settings.as_ref()).await {
                    Ok((_, output_size)) => {
                        if let Err(e) = move_original_pdf(&file_path, &settings.output_folder) {
                            let _ = app_handle.emit_all(
                                "file_error",
                                serde_json::json!({
                                    "id": file_id,
                                    "error": format!("Failed to move original: {}", e)
                                }),
                            );
                            return;
                        }

                        let _ = app_handle.emit_all(
                            "file_complete",
                            serde_json::json!({
                                "id": file_id,
                                "output_size": output_size,
                            }),
                        );
                    }
                    Err(e) => {
                        let _ = app_handle.emit_all(
                            "file_error",
                            serde_json::json!({
                                "id": file_id,
                                "error": e
                            }),
                        );
                    }
                }
            });

            handles.push(handle);
        }

        for handle in handles {
            let _ = handle.await;
        }
    });

    Ok("Processing started".to_string())
}

fn move_original_pdf(original_path: &str, destination_folder: &str) -> Result<(), String> {
    if destination_folder.is_empty() {
        return Ok(());
    }

    let original = PathBuf::from(original_path);
    let file_name = original.file_name().ok_or("Invalid file name")?;

    let destination = PathBuf::from(destination_folder).join(file_name);

    fs::rename(&original, &destination).map_err(|e| e.to_string())?;

    Ok(())
}

fn main() {
    // Load settings from disk, or use defaults if not found
    let initial_settings = settings::load_settings()
        .unwrap_or_else(|_| settings::default_settings());

    tauri::Builder::default()
        .manage(AppState {
            settings: Arc::new(Mutex::new(initial_settings)),
        })
        .invoke_handler(tauri::generate_handler![
            load_settings,
            save_settings,
            select_folder,
            process_files
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
