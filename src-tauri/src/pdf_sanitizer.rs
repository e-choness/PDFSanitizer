use crate::SanitizationSettings;
use std::path::PathBuf;
use std::fs;

pub async fn sanitize_pdf(
    input_path: &str,
    settings: &SanitizationSettings,
) -> Result<(String, u64), String> {
    let input = PathBuf::from(input_path);

    if !input.exists() {
        return Err("Input file not found".to_string());
    }

    // Read original PDF
    let pdf_bytes = fs::read(&input)
        .map_err(|e| format!("Failed to read PDF: {}", e))?;

    let mut output_bytes = pdf_bytes.clone();

    // Remove metadata if requested
    if settings.remove_metadata {
        output_bytes = remove_metadata(output_bytes)
            .unwrap_or(output_bytes);
    }

    // Remove scripts if requested
    if settings.remove_scripts {
        output_bytes = remove_scripts(output_bytes)
            .unwrap_or(output_bytes);
    }

    // Remove embedded files if requested
    if settings.remove_embedded_files {
        output_bytes = remove_embedded_files(output_bytes)
            .unwrap_or(output_bytes);
    }

    // Strip external links if requested
    if settings.strip_external_links {
        output_bytes = strip_external_links(output_bytes)
            .unwrap_or(output_bytes);
    }

    // Write sanitized PDF to original location
    let output_path = input.clone();
    fs::write(&output_path, &output_bytes)
        .map_err(|e| format!("Failed to write sanitized PDF: {}", e))?;

    let output_size = output_bytes.len() as u64;

    Ok((output_path.to_string_lossy().to_string(), output_size))
}

fn remove_metadata(pdf_bytes: Vec<u8>) -> Result<Vec<u8>, String> {
    // Remove metadata entries from PDF
    let pdf_str = String::from_utf8_lossy(&pdf_bytes);
    let mut result = pdf_str.to_string();

    // Remove common metadata fields
    result = remove_pdf_dictionary_values(&result, &[
        "/Producer",
        "/Creator",
        "/CreationDate",
        "/ModDate",
        "/Author",
        "/Subject",
        "/Title",
        "/Keywords",
    ]);

    Ok(result.into_bytes())
}

fn remove_scripts(pdf_bytes: Vec<u8>) -> Result<Vec<u8>, String> {
    // Remove JavaScript and action dictionaries from PDF
    let pdf_str = String::from_utf8_lossy(&pdf_bytes);
    let mut result = pdf_str.to_string();

    // Remove /JS, /JavaScript, /OpenAction, /AA (Additional Actions)
    result = remove_pdf_references(&result, &[
        "/JS",
        "/JavaScript",
        "/OpenAction",
        "/AA",
    ]);

    Ok(result.into_bytes())
}

fn remove_embedded_files(pdf_bytes: Vec<u8>) -> Result<Vec<u8>, String> {
    // Remove embedded file streams from PDF
    let pdf_str = String::from_utf8_lossy(&pdf_bytes);
    let mut result = pdf_str.to_string();

    // Remove /EmbeddedFile references and /EmbeddedFile entries
    result = remove_pdf_references(&result, &[
        "/EmbeddedFile",
        "/Names",
    ]);

    Ok(result.into_bytes())
}

fn strip_external_links(pdf_bytes: Vec<u8>) -> Result<Vec<u8>, String> {
    // Remove URL references and external links from PDF
    let pdf_str = String::from_utf8_lossy(&pdf_bytes);
    let mut result = pdf_str.to_string();

    // Remove /URI links and external link types
    result = result.replace("/URI (", "/URI ()");
    result = remove_pdf_references(&result, &[
        "/GoToR",
        "/Launch",
        "/GoToE",
        "/ImportData",
        "/OpenAction",
    ]);

    Ok(result.into_bytes())
}

fn remove_pdf_dictionary_values(text: &str, keys: &[&str]) -> String {
    let mut result = text.to_string();

    for key in keys {
        // Match patterns like /Key (value) or /Key [value]
        let pattern_paren = format!(r"{}\s*\([^)]*\)", key);
        let pattern_bracket = format!(r"{}\s*\[[^\]]*\]", key);

        // Simple replacement - removes the entry
        for line in result.lines() {
            if line.contains(key) {
                // This is a simplified approach
                result = result.replace(line, "");
            }
        }
    }

    result
}

fn remove_pdf_references(text: &str, refs: &[&str]) -> String {
    let mut result = text.to_string();

    for r in refs {
        // Remove reference keywords
        let pattern = format!(r"{}\s+", r);
        result = result.replace(&pattern, "");
        // Also remove the reference itself
        result = result.replace(r, "");
    }

    result
}
