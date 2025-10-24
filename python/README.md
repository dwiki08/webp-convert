# WebP Image Converter (Python Version)

🖼️ A high-performance Python tool to convert images to WebP format with advanced compression options.

## Features

- ✅ **Real WebP conversion** using Pillow library
- 🎯 **Adjustable quality settings** (1-100)
- 🔒 **Lossless compression** support
- ⚙️ **Multiple compression methods** (0-6)
- 📁 **Batch processing** with recursive directory support
- 📂 **Custom output folder** support
- ⏱️ **Detailed timing and performance metrics**
- 🎨 **Beautiful emoji-enhanced output**
- 📊 **Compression statistics**
- 🛡️ **Robust error handling**

## Installation

### Prerequisites
- Python 3.7 or higher
- pip package manager

### Install Dependencies
```bash
pip install -r requirements.txt
```

Or install manually:
```bash
pip install Pillow>=10.0.0
```

## Usage

### Basic Conversion
```bash
python convert_webp.py image.jpg
```

### Advanced Options
```bash
# High quality conversion
python convert_webp.py image.jpg -q 90

# Lossless compression
python convert_webp.py image.jpg --lossless

# Custom output path
python convert_webp.py image.jpg -o output.webp

# Verbose output
python convert_webp.py image.jpg -v

# Batch convert directory
python convert_webp.py ./images/ -r

# Maximum compression
python convert_webp.py image.jpg -q 60 -m 6

# Custom output folder
python convert_webp.py image.jpg --output-folder ./converted

# Batch convert to custom folder
python convert_webp.py ./images/ --output-folder ./webp_files -r
```

## Command Line Options

| Option | Description | Default |
|--------|-------------|---------|
| `input` | Input image file or directory | Required |
| `-o, --output` | Output file path (single file) | Auto-generated |
| `-q, --quality` | Quality 1-100 | `80` |
| `--lossless` | Use lossless compression | `False` |
| `-m, --method` | Compression method 0-6 | `4` |
| `-r, --recursive` | Process subdirectories | `False` |
| `-v, --verbose` | Show detailed information | `False` |
| `--output-folder` | Custom output folder for converted images | Same as input |

## Supported Formats

### Input Formats
- JPEG (.jpg, .jpeg)
- PNG (.png)
- BMP (.bmp)
- TIFF (.tiff, .tif)
- GIF (.gif)

### Output Format
- WebP (.webp)

## Examples

### Single File Conversion
```bash
$ python convert_webp.py sample.png -v
🖼️  WebP Image Converter - Python Version
==================================================
📂 Input: sample.png
🎯 Quality: 80%
🔒 Lossless: False
⚙️  Method: 4
📁 Recursive: False
==================================================
🔍 Found 1 image(s) to convert...
==================================================
✅ Converted: sample.png
   📁 Output: sample.webp
   📊 Original: 12.0 MB
   🗜️  Compressed: 853.3 KB
   📈 Compression: 93.0%
   ⏱️  Time taken: 2.45s

🎉 Conversion completed successfully in 2.45s!
```

### Batch Conversion
```bash
$ python convert_webp.py ./photos/ -r -q 85
🖼️  WebP Image Converter - Python Version
==================================================
🔍 Found 25 image(s) to convert...
==================================================
✅ Converted: IMG_001.jpg
   📁 Output: IMG_001.webp
   📊 Original: 3.2 MB
   🗜️  Compressed: 412.1 KB
   📈 Compression: 87.2%
   ⏱️  Time taken: 0.89s

... (more conversions) ...

============================================================
📊 Summary: 25 images converted successfully
⏱️  Total time: 18.45s
📈 Average time per image: 0.74s
```

### Custom Output Folder Examples

#### Single File to Custom Folder
```bash
$ python convert_webp.py sample.png --output-folder ./webp_output -v
🖼️  WebP Image Converter - Python Version
==================================================
📂 Input: sample.png
🎯 Quality: 80%
🔒 Lossless: False
⚙️  Method: 4
📁 Recursive: False
📂 Output folder: webp_output
==================================================
✅ Converted: sample.png
   📁 Output: sample.webp
   📊 Original: 12.0 MB
   🗜️  Compressed: 853.3 KB
   📈 Compression: 93.0%
   ⏱️  Time taken: 2.45s

🎉 Conversion completed successfully in 2.45s!
```

#### Batch Conversion to Custom Folder
```bash
$ python convert_webp.py ./photos/ --output-folder ./converted_photos -r -q 85
🖼️  WebP Image Converter - Python Version
==================================================
📂 Input: ./photos/
🎯 Quality: 85%
🔒 Lossless: False
⚙️  Method: 4
📁 Recursive: True
📂 Output folder: converted_photos
==================================================
🔍 Found 15 image(s) to convert...
==================================================
✅ Converted: IMG_001.jpg
   📁 Output: IMG_001.webp
   📊 Original: 3.2 MB
   🗜️  Compressed: 412.1 KB
   📈 Compression: 87.2%
   ⏱️  Time taken: 0.89s

... (more conversions) ...

============================================================
📊 Summary: 15 images converted successfully
⏱️  Total time: 12.45s
📈 Average time per image: 0.83s
```

#### Key Features of Custom Output Folder:
- ✅ **Auto-creation**: Output folder is created automatically if it doesn't exist
- ✅ **Clean organization**: Keeps converted files separate from originals
- ✅ **Preserves filenames**: Original filenames maintained with .webp extension
- ✅ **Works with all options**: Combines with quality, method, recursive, etc.
- ✅ **Single & batch**: Works for individual files and directory processing

## Performance Tips

### Quality vs File Size
- **High quality (90-100)**: Best visual quality, larger files
- **Medium quality (70-89)**: Good balance of quality and size
- **Low quality (1-69)**: Smallest files, reduced visual quality

### Compression Methods
- **Method 0**: Fastest compression, larger files
- **Method 4**: Balanced compression (recommended)
- **Method 6**: Best compression, slowest processing

### Lossless vs Lossy
- **Lossless**: Perfect quality, larger files (ideal for archival)
- **Lossy**: Smaller files, minimal quality loss (ideal for web)

## Error Handling

The converter includes comprehensive error handling for:
- Invalid file formats
- Corrupted images
- Permission issues
- Disk space problems
- Unsupported compression settings

## Development

### Project Structure
```
python/
├── convert_webp.py    # Main converter application
├── requirements.txt   # Python dependencies
├── README.md         # This documentation
├── sample.png        # Sample image for testing
└── *.webp           # Generated WebP files
```

### Code Quality
- ✅ Type hints with `typing` module
- ✅ Comprehensive docstrings
- ✅ Class-based architecture
- ✅ Error handling with try-catch blocks
- ✅ Clean, readable code with PEP 8 compliance

## Troubleshooting

### Common Issues

**Q: "PIL not found" error**
```bash
pip install Pillow
```

**Q: Permission denied errors**
- Check file/directory permissions
- Run with appropriate user privileges

**Q: Out of memory errors**
- Reduce image size before conversion
- Use lower quality settings
- Process images in smaller batches

### Performance Optimization

1. **Use appropriate quality settings** - Higher quality = larger files
2. **Choose right compression method** - Method 4 is usually optimal
3. **Batch process efficiently** - Use recursive mode for large directories
4. **Use custom output folders** - Organize converted files and avoid cluttering original directories
5. **Monitor system resources** - Ensure adequate RAM and disk space

## License

This project is open source and available under the MIT License.

## Contributing

Contributions are welcome! Please feel free to submit pull requests or report issues.

---

**Happy converting! 🚀**