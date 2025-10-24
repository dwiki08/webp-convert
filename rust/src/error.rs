//! Error handling module for the WebP converter.

use thiserror::Error;
use std::path::PathBuf;

/// Custom error types for the WebP converter.
#[derive(Error, Debug)]
pub enum WebPError {
    #[error("Input file or directory not found: {0}")]
    InputNotFound(PathBuf),

    #[error("Invalid input type (neither file nor directory): {0}")]
    InvalidInputType(PathBuf),

    #[error("Invalid or unsupported image file: {0}")]
    InvalidImage(PathBuf),

    #[error("Invalid file name: {0}")]
    InvalidFileName(PathBuf),

    #[error("Image processing failed: {0}")]
    ImageProcessingError(String),

    #[error("Encoding failed: {0}")]
    EncodingError(String),

    #[error("File I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Directory traversal error: {0}")]
    WalkDirError(#[from] walkdir::Error),

    #[error("No supported image files found in directory")]
    NoImagesFound,
}

/// Type alias for Result with our custom error type.
pub type WebPResult<T> = Result<T, WebPError>;