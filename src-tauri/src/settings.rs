use crate::SanitizationSettings;
use std::path::PathBuf;
use std::fs;

const SETTINGS_FILE: &str = "settings.json";

pub fn save_settings(settings: &SanitizationSettings) -> Result<(), String> {
    let json = serde_json::to_string_pretty(settings)
        .map_err(|e| e.to_string())?;

    let path = get_settings_path();
    fs::write(path, json)
        .map_err(|e| e.to_string())?;

    Ok(())
}

pub fn load_settings() -> Result<SanitizationSettings, String> {
    let path = get_settings_path();

    if !path.exists() {
        return Ok(default_settings());
    }

    let json = fs::read_to_string(path)
        .map_err(|e| e.to_string())?;

    serde_json::from_str(&json)
        .map_err(|e| e.to_string())
}

fn get_settings_path() -> PathBuf {
    let config_dir = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."));

    config_dir.join("pdf-sanitizer").join(SETTINGS_FILE)
}

pub fn default_settings() -> SanitizationSettings {
    SanitizationSettings {
        remove_metadata: true,
        remove_scripts: true,
        remove_embedded_files: true,
        compress_images: false,
        high_compression: false,
        strip_external_links: false,
        font_subsetting: false,
        max_concurrent: 4,
        output_folder: String::new(),
    }
}
