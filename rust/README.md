# 🦀 WebP Image Converter (Rust Version)

A high-performance WebP image converter built with Rust, featuring memory-safe operations, excellent performance, and a modern command-line interface.

## ✨ Features

- 🚀 **High Performance**: Built with Rust for maximum speed and efficiency
- 🛡️ **Memory Safety**: Rust's ownership system prevents common memory errors
- 📦 **Single Binary**: No runtime dependencies, just one executable
- 🎯 **Adjustable Quality**: Fine-tune compression quality (1-100)
- 🔒 **Lossless Compression**: Support for both lossy and lossless compression
- ⚙️ **Compression Methods**: 7 different compression levels (0-6)
- 📁 **Batch Processing**: Convert entire directories with recursive support
- ⏱️ **Performance Metrics**: Detailed timing and compression statistics
- 🎨 **Beautiful Output**: Emoji-enhanced, colorized terminal output
- 📊 **Comprehensive Stats**: Track conversion success rates and file sizes

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))
- For WebP encoding: Visual Studio Build Tools (Windows) or build tools for your platform

### Installation

```bash
# Clone the repository
git clone https://github.com/your-username/webp-converter
cd webp-converter/rust

# Build the project
cargo build --release

# The binary will be available at target/release/webp-converter
```

### Basic Usage

```bash
# Convert a single image
./target/release/webp-converter image.jpg

# High quality conversion
./target/release/webp-converter image.jpg -q 90

# Lossless compression
./target/release/webp-converter image.jpg --lossless

# Batch convert directory
./target/release/webp-converter ./photos/ -r

# Verbose output
./target/release/webp-converter image.jpg -v
```

## 📖 Command Line Options

| Option | Short | Long | Description | Default |
|--------|-------|------|-------------|---------|
| Input | - | `input` | Input image file or directory | Required |
| Output | `-o` | `--output` | Output file path (single file) | Auto-generated |
| Quality | `-q` | `--quality` | Quality 1-100 | `80` |
| Lossless | - | `--lossless` | Use lossless compression | `false` |
| Method | `-m` | `--method` | Compression method 0-6 | `4` |
| Recursive | `-r` | `--recursive` | Process subdirectories | `false` |
| Verbose | `-v` | `--verbose` | Show detailed information | `false` |

## 🏗️ Project Structure

```
rust/
├── Cargo.toml              # Project configuration
├── Cargo.lock              # Dependency lock file
├── README.md               # This documentation
├── src/                    # Source code
│   ├── main.rs            # Main application entry point
│   ├── converter.rs       # Core conversion logic
│   ├── error.rs           # Error handling
│   └── utils.rs           # Utility functions
├── target/                 # Build output directory
└── convert_webp_standalone.rs  # Standalone demo version
```

## 🔧 Development

### Building

```bash
# Debug build (faster compilation)
cargo build

# Release build (optimized for performance)
cargo build --release

# Run tests
cargo test

# Check code without building
cargo check
```

## ⚠️ Important Notes

### WebP Encoding Status

This version currently uses PNG compression as a fallback to demonstrate the converter functionality. For production WebP encoding, you have several options:

1. **Use the `webp` crate**:
   ```toml
   [dependencies]
   webp = "0.2"
   ```

2. **Use `libwebp-sys` bindings** for maximum performance

### Platform Requirements

- **Windows**: Visual Studio Build Tools with C++ support
- **Linux**: `build-essential` package
- **macOS**: Xcode Command Line Tools

## 🤝 Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Commit your changes: `git commit -m 'Add amazing feature'`
4. Push to the branch: `git push origin feature/amazing-feature`
5. Open a pull request

## 📄 License

This project is licensed under the MIT License.

---

**Happy converting! 🦀✨**