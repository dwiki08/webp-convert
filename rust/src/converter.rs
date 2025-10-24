//! Main WebP converter module.

use crate::error::{WebPError, WebPResult};
use crate::utils;
use std::path::{Path, PathBuf};
use std::time::Instant;
use std::fs;
use walkdir::WalkDir;

/// Statistics for conversion operations.
#[derive(Debug, Default)]
pub struct ConversionStats {
    pub success_count: usize,
    pub failed_count: usize,
    pub total_time: f64,
    pub total_original_size: Option<u64>,
    pub total_compressed_size: Option<u64>,
}

impl ConversionStats {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_success(&mut self, time_taken: f64, original_size: u64, compressed_size: u64) {
        self.success_count += 1;
        self.total_time += time_taken;
        self.total_original_size = Some(self.total_original_size.unwrap_or(0) + original_size);
        self.total_compressed_size = Some(self.total_compressed_size.unwrap_or(0) + compressed_size);
    }

    pub fn add_failure(&mut self) {
        self.failed_count += 1;
    }
}

/// Main WebP converter.
pub struct WebPConverter {
    quality: u8,
    lossless: bool,
    method: u8,
}

impl WebPConverter {
    /// Create a new WebP converter with specified settings.
    pub fn new(quality: u8, lossless: bool, method: u8) -> Self {
        Self {
            quality,
            lossless,
            method,
        }
    }

    /// Convert a single image file to WebP.
    pub fn convert_single_file(
        &self,
        input_path: &Path,
        output_path: Option<&Path>,
    ) -> WebPResult<ConversionStats> {
        // Validate input file
        if !utils::is_valid_image(input_path) {
            return Err(WebPError::InvalidImage(input_path.to_path_buf()));
        }

        // Generate output path if not provided
        let generated_path = utils::generate_output_path(input_path);
        let output_path = output_path.unwrap_or(&generated_path);

        // Perform conversion
        let (time_taken, original_size, compressed_size) =
            self.convert_image_to_webp(input_path, output_path)?;

        // Create and return stats
        let mut stats = ConversionStats::new();
        stats.add_success(time_taken, original_size, compressed_size);

        Ok(stats)
    }

    /// Convert all images in a directory to WebP.
    pub fn convert_directory(
        &self,
        directory: &Path,
        recursive: bool,
    ) -> WebPResult<ConversionStats> {
        if !directory.exists() {
            return Err(WebPError::InputNotFound(directory.to_path_buf()));
        }

        // Find all image files
        let image_files = self.find_image_files(directory, recursive)?;

        if image_files.is_empty() {
            return Err(WebPError::NoImagesFound);
        }

        println!("🔍 Found {} image(s) to convert...", image_files.len());
        println!("{}", "=".repeat(60));

        let mut stats = ConversionStats::new();

        for img_file in &image_files {
            // Skip if already WebP
            if utils::is_webp_file(img_file) {
                println!("⏭️  Skipping {} (already WebP)", img_file.file_name().unwrap_or_default().to_string_lossy());
                continue;
            }

            // Convert the image
            let output_path = utils::generate_output_path(img_file);
            match self.convert_image_to_webp(img_file, &output_path) {
                Ok((time_taken, original_size, compressed_size)) => {
                    stats.add_success(time_taken, original_size, compressed_size);
                }
                Err(e) => {
                    eprintln!("❌ Error converting {}: {}", img_file.display(), e);
                    stats.add_failure();
                }
            }
        }

        Ok(stats)
    }

    /// Convert an image to WebP format.
    fn convert_image_to_webp(
        &self,
        input_path: &Path,
        output_path: &Path,
    ) -> WebPResult<(f64, u64, u64)> {
        let start_time = Instant::now();

        // Load the image
        let img = image::open(input_path)
            .map_err(|e| WebPError::ImageProcessingError(format!("Failed to open image: {}", e)))?;

        // Convert to RGB if necessary
        let rgb_img = img.to_rgb8();

        // Encode to WebP
        let webp_data = self.encode_to_webp(&rgb_img)?;

        // Write to file
        fs::write(output_path, webp_data)
            .map_err(|e| WebPError::IoError(e))?;

        // Calculate timing
        let time_taken = start_time.elapsed().as_secs_f64();

        // Get file sizes
        let original_size = fs::metadata(input_path)?.len();
        let compressed_size = fs::metadata(output_path)?.len();
        let compression_ratio = (1.0 - compressed_size as f64 / original_size as f64) * 100.0;

        // Print conversion results
        self.print_conversion_result(
            input_path,
            output_path,
            original_size,
            compressed_size,
            compression_ratio,
            time_taken,
        );

        Ok((time_taken, original_size, compressed_size))
    }

    /// Encode RGB image to WebP format.
    fn encode_to_webp(&self, img: &image::RgbImage) -> WebPResult<Vec<u8>> {
        use webp::Encoder;

        // Convert image to RGB bytes
        let (width, height) = img.dimensions();
        let rgb_data = img.as_raw();

        // Create WebP encoder with quality settings
        let encoder = Encoder::from_rgb(
            rgb_data,
            width as u32,
            height as u32,
        );

        // Set quality based on settings
        let quality = if self.lossless { 100.0 } else { self.quality as f32 };

        // Encode to WebP
        let webp_data = encoder.encode(quality);

        // Check if encoding was successful by trying to access the data
        if webp_data.len() > 0 {
            Ok(webp_data.to_vec())
        } else {
            Err(WebPError::EncodingError("Failed to encode WebP - empty result".to_string()))
        }
    }

    /// Print formatted conversion results.
    fn print_conversion_result(
        &self,
        input_path: &Path,
        output_path: &Path,
        original_size: u64,
        compressed_size: u64,
        compression_ratio: f64,
        time_taken: f64,
    ) {
        println!("✅ Converted: {}", input_path.file_name().unwrap_or_default().to_string_lossy());
        println!("   📁 Output: {}", output_path.file_name().unwrap_or_default().to_string_lossy());
        println!("   📊 Original: {}", utils::format_size(original_size));
        println!("   🗜️  Compressed: {}", utils::format_size(compressed_size));
        println!("   📈 Compression: {:.1}%", compression_ratio);
        println!("   ⏱️  Time taken: {:.2}s", time_taken);
        println!();
    }

    /// Find all image files in directory.
    fn find_image_files(&self, directory: &Path, recursive: bool) -> WebPResult<Vec<PathBuf>> {
        let mut image_files = Vec::new();

        if recursive {
            for entry in WalkDir::new(directory) {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() && utils::is_supported_extension(path) {
                    image_files.push(path.to_path_buf());
                }
            }
        } else {
            for entry in fs::read_dir(directory)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() && utils::is_supported_extension(&path) {
                    image_files.push(path);
                }
            }
        }

        Ok(image_files)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_converter_creation() {
        let converter = WebPConverter::new(80, false, 4);
        assert_eq!(converter.quality, 80);
        assert!(!converter.lossless);
        assert_eq!(converter.method, 4);
    }

    #[test]
    fn test_conversion_stats() {
        let mut stats = ConversionStats::new();
        stats.add_success(1.5, 1000, 200);
        stats.add_success(2.0, 1500, 300);
        stats.add_failure();

        assert_eq!(stats.success_count, 2);
        assert_eq!(stats.failed_count, 1);
        assert_eq!(stats.total_time, 3.5);
        assert_eq!(stats.total_original_size, Some(2500));
        assert_eq!(stats.total_compressed_size, Some(500));
    }
}