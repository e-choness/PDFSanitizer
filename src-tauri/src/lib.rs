pub mod pdf_sanitizer;
pub mod settings;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
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
