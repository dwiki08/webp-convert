//! Utility functions for the WebP converter.

use std::path::Path;

/// Format file size in human-readable format.
pub fn format_size(size_bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = size_bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.1} {}", size, UNITS[unit_index])
}

/// Check if a file is a valid image format.
pub fn is_valid_image(file_path: &Path) -> bool {
    image::open(file_path).is_ok()
}

/// Get supported image extensions.
pub fn supported_extensions() -> &'static [&'static str] {
    &["jpg", "jpeg", "png", "bmp", "tiff", "tif", "gif"]
}

/// Check if file has supported image extension.
pub fn is_supported_extension(file_path: &Path) -> bool {
    if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
        supported_extensions().contains(&ext.to_lowercase().as_str())
    } else {
        false
    }
}

/// Check if file is already WebP format.
pub fn is_webp_file(file_path: &Path) -> bool {
    if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
        ext.to_lowercase() == "webp"
    } else {
        false
    }
}

/// Generate output path for WebP conversion.
pub fn generate_output_path(input_path: &Path) -> std::path::PathBuf {
    input_path.with_extension("webp")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(512), "512.0 B");
        assert_eq!(format_size(1536), "1.5 KB");
        assert_eq!(format_size(1048576), "1.0 MB");
        assert_eq!(format_size(1073741824), "1.0 GB");
    }

    #[test]
    fn test_is_webp_file() {
        assert!(is_webp_file(Path::new("test.webp")));
        assert!(is_webp_file(Path::new("test.WEBP")));
        assert!(!is_webp_file(Path::new("test.jpg")));
        assert!(!is_webp_file(Path::new("test")));
    }

    #[test]
    fn test_is_supported_extension() {
        assert!(is_supported_extension(Path::new("test.jpg")));
        assert!(is_supported_extension(Path::new("test.png")));
        assert!(is_supported_extension(Path::new("test.tiff")));
        assert!(!is_supported_extension(Path::new("test.webp")));
        assert!(!is_supported_extension(Path::new("test.txt")));
    }
}