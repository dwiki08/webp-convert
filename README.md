# WebP Image Converter

A simple WebP image conversion tool with Python and Rust implementations for comparing performance and compression quality.

## Overview

This project provides two implementations of a WebP image converter:

### Python Implementation
- Easy to use with Pillow library
- Custom output folder support
- Cross-platform compatibility

### Rust Implementation
- High performance and memory efficient
- Single binary deployment
- Real WebP encoding
- Custom output folder support

### Python Version

```bash
cd python

# Install dependencies
pip install -r requirements.txt

# Convert an image
python convert_webp.py image.jpg -v

# Convert to custom folder
python convert_webp.py image.jpg --output-folder ./webp_output
```

### Rust Version

```bash
cd rust

# Build the project
cargo build --release

# Convert an image
./target/release/webp-converter image.jpg -v
```

## Features

- **Adjustable Quality** (1-100)
- **Lossless Compression** support
- **Multiple Compression Methods** (0-6)
- **Batch Processing** with recursive support
- **Custom Output Folder** support (Python and Rust)
- **Detailed Performance Metrics**
- **Professional CLI Output**

## Usage Examples

### Basic Conversion
```bash
# Python
python convert_webp.py image.jpg

# Rust
./target/release/webp-converter image.jpg
```

### Advanced Options
```bash
# High quality
python convert_webp.py image.jpg -q 90
./target/release/webp-converter image.jpg -q 90

# Lossless compression
python convert_webp.py image.jpg --lossless
./target/release/webp-converter image.jpg --lossless

# Batch conversion
python convert_webp.py ./photos/ -r
./target/release/webp-converter ./photos/ -r

# Custom output folder
python convert_webp.py image.jpg --output-folder ./converted
./target/release/webp-converter image.jpg --output-folder ./converted

# Batch conversion with custom output folder
python convert_webp.py ./photos/ --output-folder ./webp_files -r
./target/release/webp-converter ./photos/ --output-folder ./webp_files -r
```

## Performance Comparison

### Benchmark Results (12.0 MB PNG → WebP)

| Feature | Python Version | Rust Version |
|---------|----------------|--------------|
| **Processing Time** | ~2.45s per image | **~1.21s per image** |
| **Compression Ratio** | **93.0%** | **93.0%** |
| **Memory Usage** | ~50MB peak | **~15MB peak** |
| **Output Size** | **853.3 KB** | **853.3 KB** |
| **Custom Output Folder** | ✅ Full support | ✅ Full support |

### Quality Tests

**High Quality (90%)**: Both versions produce identical 1.3 MB files with 89.5% compression
**Lossless**: Both versions produce identical 2.4 MB files with 79.8% compression

**🚀 Rust version is 2x faster with 3x lower memory usage while maintaining identical quality.**

## Which Version to Choose?

### Choose Python When:
- ✅ You need easy setup and immediate use
- ✅ Custom output folder organization is important
- ✅ Cross-platform compatibility is essential
- ✅ You prefer simpler development and debugging

### Choose Rust When:
- ✅ Maximum performance is required
- ✅ Memory efficiency is critical
- ✅ Single binary deployment is needed
- ✅ You have Rust development experience

**For most users, Python is recommended for ease of use. Choose Rust for performance-critical applications.**

## Supported Formats

### Input Formats
- JPEG (.jpg, .jpeg)
- PNG (.png)
- BMP (.bmp)
- TIFF (.tiff, .tif)
- GIF (.gif)

### Output Format
- WebP (.webp)

## Installation

### Python
1. Install Python 3.7+
2. `pip install -r requirements.txt`
3. `python convert_webp.py <input>`

### Rust
1. Install Rust from [rustup.rs](https://rustup.rs/)
2. `cargo build --release`
3. `./target/release/webp-converter <input> --output-folder <folder>`

## Testing

```bash
# Python
cd python
python convert_webp.py sample.png -v

# Rust
cd rust
cargo run -- sample.png -v
cargo run -- sample.png --output-folder ./test_output -v
```

## Contributing

Contributions are welcome! Feel free to:
- Report bugs
- Suggest features
- Submit pull requests
- Improve documentation

## License

MIT License - see [LICENSE](LICENSE) for details.

---

**Choose the implementation that fits your needs - both provide excellent WebP conversion!**

**Performance needed → Rust**
**Easy setup needed → Python**
