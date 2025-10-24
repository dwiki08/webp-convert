//! 🦀 WebP Image Converter - Rust Version
//!
//! A high-performance WebP image converter built with Rust, featuring
//! memory-safe operations and excellent performance characteristics.

mod converter;
mod error;
mod utils;

use clap::Parser;
use std::path::PathBuf;
use anyhow::Result;

use crate::converter::WebPConverter;
use crate::error::WebPError;

#[derive(Parser)]
#[command(
    name = "webp-converter",
    version = "1.0.0",
    author = "WebP Converter Team",
    about = "A high-performance WebP image converter built with Rust",
    long_about = "Convert images to WebP format with advanced compression options and excellent performance."
)]
#[command(help_expected = true)]
struct Args {
    /// Input image file or directory
    #[arg(help = "Input image file or directory to process")]
    input: PathBuf,

    /// Output file path (for single file conversion)
    #[arg(
        short = 'o',
        long = "output",
        help = "Output file path (only used for single file conversion)"
    )]
    output: Option<PathBuf>,

    /// Quality setting (1-100)
    #[arg(
        short = 'q',
        long = "quality",
        help = "Quality setting from 1 (lowest) to 100 (highest)",
        default_value = "80",
        value_parser = clap::value_parser!(u8).range(1..=100)
    )]
    quality: u8,

    /// Use lossless compression
    #[arg(
        long = "lossless",
        help = "Use lossless compression instead of lossy"
    )]
    lossless: bool,

    /// Compression method (0-6)
    #[arg(
        short = 'm',
        long = "method",
        help = "Compression method: 0 (fastest) to 6 (best compression)",
        default_value = "4",
        value_parser = clap::value_parser!(u8).range(0..=6)
    )]
    method: u8,

    /// Process subdirectories recursively
    #[arg(
        short = 'r',
        long = "recursive",
        help = "Process subdirectories when input is a directory"
    )]
    recursive: bool,

    /// Enable verbose output
    #[arg(
        short = 'v',
        long = "verbose",
        help = "Show detailed conversion information"
    )]
    verbose: bool,
}

fn main() -> Result<()> {
    print_banner();

    let args = Args::parse();

    // Validate input path
    if !args.input.exists() {
        return Err(WebPError::InputNotFound(args.input.clone()).into());
    }

    // Show verbose information
    if args.verbose {
        print_verbose_info(&args);
    }

    // Create converter instance
    let converter = WebPConverter::new(args.quality, args.lossless, args.method);

    // Process input based on type
    let result = if args.input.is_file() {
        // Single file conversion
        converter.convert_single_file(&args.input, args.output.as_deref())
    } else if args.input.is_dir() {
        // Directory batch conversion
        if args.output.is_some() {
            println!("⚠️  Warning: Output path is ignored when processing directories");
        }
        converter.convert_directory(&args.input, args.recursive)
    } else {
        return Err(WebPError::InvalidInputType(args.input.clone()).into());
    };

    match result {
        Ok(stats) => {
            print_success_summary(&stats);
        }
        Err(e) => {
            eprintln!("❌ Conversion failed: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}

fn print_banner() {
    println!("🦀 WebP Image Converter - Rust Version");
    println!("{}", "=".repeat(50));
}

fn print_verbose_info(args: &Args) {
    println!("📂 Input: {}", args.input.display());
    if let Some(output) = &args.output {
        println!("📁 Output: {}", output.display());
    }
    println!("🎯 Quality: {}%", args.quality);
    println!("🔒 Lossless: {}", args.lossless);
    println!("⚙️  Method: {}", args.method);
    println!("📁 Recursive: {}", args.recursive);
    println!("{}", "=".repeat(50));
}

fn print_success_summary(stats: &crate::converter::ConversionStats) {
    println!("{}", "=".repeat(60));
    println!("📊 Conversion Summary:");
    println!("✅ Successfully converted: {} files", stats.success_count);
    if stats.failed_count > 0 {
        println!("❌ Failed conversions: {} files", stats.failed_count);
    }
    println!("⏱️  Total time: {:.2}s", stats.total_time);

    if stats.success_count > 0 {
        println!("📈 Average time per image: {:.2}s",
                stats.total_time / stats.success_count as f64);
    }

    if let Some(total_original) = stats.total_original_size {
        if let Some(total_compressed) = stats.total_compressed_size {
            let compression_ratio = (1.0 - total_compressed as f64 / total_original as f64) * 100.0;
            println!("🗜️  Overall compression: {:.1}%", compression_ratio);
            println!("📦 Original size: {}", crate::utils::format_size(total_original));
            println!("📦 Compressed size: {}", crate::utils::format_size(total_compressed));
        }
    }

    println!("🎉 All operations completed successfully!");
}