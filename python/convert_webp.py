#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
WebP Image Converter - Python Version
A high-performance tool to convert images to WebP format with adjustable quality settings.

Author: WebP Converter Team
Version: 1.0.0
"""

import os
import sys
import argparse
import time
from pathlib import Path
from typing import Tuple, Optional, List
from PIL import Image


class WebPConverter:
    """Main WebP converter class with all conversion functionality."""

    SUPPORTED_EXTENSIONS = {'.jpg', '.jpeg', '.png', '.bmp', '.tiff', '.tif', '.gif'}

    def __init__(self):
        self.converted_count = 0
        self.failed_count = 0

    @staticmethod
    def format_size(size_bytes: int) -> str:
        """Format file size in human readable format."""
        units = ['B', 'KB', 'MB', 'GB', 'TB']
        size = float(size_bytes)
        unit_index = 0

        while size >= 1024.0 and unit_index < len(units) - 1:
            size /= 1024.0
            unit_index += 1

        return f"{size:.1f} {units[unit_index]}"

    @staticmethod
    def is_valid_image(file_path: Path) -> bool:
        """Check if file is a supported image format."""
        try:
            with Image.open(file_path) as img:
                img.verify()
            return True
        except Exception:
            return False

    def convert_to_webp(
        self,
        input_path: Path,
        output_path: Optional[Path] = None,
        quality: int = 80,
        lossless: bool = False,
        method: int = 4,
        output_folder: Optional[Path] = None
    ) -> Tuple[Optional[str], float]:
        """
        Convert image to WebP format.

        Args:
            input_path: Path to input image
            output_path: Path to output WebP file (optional)
            quality: Quality 1-100 (default 80)
            lossless: Use lossless compression (default False)
            method: Compression method 0-6 (default 4)
            output_folder: Custom output folder (optional)

        Returns:
            Tuple of (output_path, time_taken_seconds)
        """
        start_time = time.time()

        try:
            # Generate output path if not provided
            if output_path is None:
                if output_folder:
                    # Create output folder if it doesn't exist
                    output_folder.mkdir(parents=True, exist_ok=True)
                    output_path = output_folder / f"{input_path.stem}.webp"
                else:
                    output_path = input_path.parent / f"{input_path.stem}.webp"

            # Open input image
            with Image.open(input_path) as img:
                # Convert RGBA to RGB if necessary for WebP
                if img.mode in ('RGBA', 'LA'):
                    # Create white background
                    background = Image.new('RGB', img.size, (255, 255, 255))
                    if img.mode == 'RGBA':
                        background.paste(img, mask=img.split()[-1])
                    else:
                        background.paste(img, mask=img.split()[-1])
                    img = background
                elif img.mode not in ('RGB', 'L'):
                    img = img.convert('RGB')

                # Configure WebP saving options
                save_kwargs = {
                    'format': 'WEBP',
                    'quality': quality,
                    'method': method,
                    'optimize': True
                }

                if lossless:
                    save_kwargs['lossless'] = True

                # Save as WebP
                img.save(output_path, **save_kwargs)

            # Calculate compression stats
            end_time = time.time()
            time_taken = end_time - start_time

            original_size = input_path.stat().st_size
            compressed_size = output_path.stat().st_size
            compression_ratio = (1 - compressed_size / original_size) * 100

            # Print conversion results
            self._print_conversion_result(
                input_path, output_path, original_size,
                compressed_size, compression_ratio, time_taken
            )

            self.converted_count += 1
            return str(output_path), time_taken

        except Exception as e:
            print(f"Error converting {input_path}: {str(e)}")
            self.failed_count += 1
            return None, 0.0

    def _print_conversion_result(
        self,
        input_path: Path,
        output_path: Path,
        original_size: int,
        compressed_size: int,
        compression_ratio: float,
        time_taken: float
    ) -> None:
        """Print formatted conversion results."""
        print(f"Converted: {input_path.name}")
        print(f"  Output: {output_path.name}")
        print(f"  Original: {self.format_size(original_size)}")
        print(f"  Compressed: {self.format_size(compressed_size)}")
        print(f"  Compression: {compression_ratio:.1f}%")
        print(f"  Time taken: {time_taken:.2f}s")
        print()

    def batch_convert(
        self,
        directory: Path,
        quality: int = 80,
        lossless: bool = False,
        method: int = 4,
        recursive: bool = False,
        output_folder: Optional[Path] = None
    ) -> Tuple[int, float]:
        """
        Convert all images in directory to WebP.

        Args:
            directory: Directory path
            quality: Quality setting
            lossless: Use lossless compression
            method: Compression method
            recursive: Process subdirectories
            output_folder: Custom output folder

        Returns:
            Tuple of (success_count, total_time)
        """
        if not directory.exists():
            raise FileNotFoundError(f"Directory does not exist: {directory}")

        # Find all image files
        image_files = self._find_image_files(directory, recursive)

        if not image_files:
            print("No supported image files found.")
            return 0, 0.0

        print(f"Found {len(image_files)} image(s) to convert...")
        print("=" * 60)

        total_time = 0.0

        for img_file in image_files:
            # Skip if already WebP
            if img_file.suffix.lower() == '.webp':
                print(f"Skipping {img_file.name} (already WebP)")
                continue

            _, time_taken = self.convert_to_webp(img_file, None, quality, lossless, method, output_folder)
            total_time += time_taken

        return self.converted_count, total_time

    def _find_image_files(self, directory: Path, recursive: bool) -> List[Path]:
        """Find all image files in directory."""
        image_files = []

        if recursive:
            pattern = "**/*"
        else:
            pattern = "*"

        for file_path in directory.glob(pattern):
            if (file_path.is_file() and
                file_path.suffix.lower() in self.SUPPORTED_EXTENSIONS):
                image_files.append(file_path)

        return sorted(image_files)


def create_argument_parser() -> argparse.ArgumentParser:
    """Create and configure argument parser."""
    parser = argparse.ArgumentParser(
        description="Convert images to WebP format with compression options",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  %(prog)s image.jpg                    # Convert with default settings
  %(prog)s image.jpg -q 90             # High quality (90%%)
  %(prog)s image.jpg --lossless        # Lossless compression
  %(prog)s ./images/ -r                # Batch convert directory
  %(prog)s image.jpg -o output.webp    # Custom output path
  %(prog)s image.jpg --output-folder ./out  # Convert to custom folder
  %(prog)s ./images/ --output-folder ./converted  # Batch convert to custom folder
        """
    )

    parser.add_argument(
        'input',
        help='Input image file or directory'
    )

    parser.add_argument(
        '-o', '--output',
        help='Output file path (for single file)'
    )

    parser.add_argument(
        '-q', '--quality',
        type=int,
        default=80,
        choices=range(1, 101),
        help='Quality 1-100 (default: 80)'
    )

    parser.add_argument(
        '--lossless',
        action='store_true',
        help='Use lossless compression'
    )

    parser.add_argument(
        '-m', '--method',
        type=int,
        default=4,
        choices=range(7),
        help='Compression method 0-6 (default: 4, higher = smaller but slower)'
    )

    parser.add_argument(
        '-r', '--recursive',
        action='store_true',
        help='Process subdirectories when input is a directory'
    )

    parser.add_argument(
        '-v', '--verbose',
        action='store_true',
        help='Show detailed information'
    )

    parser.add_argument(
        '--output-folder',
        help='Output folder for converted images (e.g., ./out)'
    )

    return parser


def main() -> None:
    """Main entry point."""
    print("WebP Image Converter - Python Version")
    print("=" * 50)

    parser = create_argument_parser()
    args = parser.parse_args()

    input_path = Path(args.input)
    output_path = Path(args.output) if args.output else None
    output_folder = Path(args.output_folder) if args.output_folder else None
    quality = args.quality
    lossless = args.lossless
    method = args.method
    recursive = args.recursive
    verbose = args.verbose

    # Validate input path
    if not input_path.exists():
        print(f"Error: Input path does not exist: {input_path}")
        sys.exit(1)

    # Show verbose information
    if verbose:
        print(f"Input: {input_path}")
        print(f"Quality: {quality}%")
        print(f"Lossless: {lossless}")
        print(f"Method: {method}")
        print(f"Recursive: {recursive}")
        if output_folder:
            print(f"Output folder: {output_folder}")
        print("=" * 50)

    # Initialize converter
    converter = WebPConverter()

    try:
        if input_path.is_file():
            # Single file conversion
            if not converter.is_valid_image(input_path):
                print(f"Error: Invalid or unsupported image file: {input_path}")
                sys.exit(1)

            result, time_taken = converter.convert_to_webp(
                input_path, output_path, quality, lossless, method, output_folder
            )

            if result:
                print(f"Conversion completed successfully in {time_taken:.2f}s!")
            else:
                print(f"Conversion failed!")
                sys.exit(1)

        elif input_path.is_dir():
            # Directory batch conversion
            if output_path:
                print("Warning: Output path is ignored when processing directories")
                print()

            success_count, total_time = converter.batch_convert(
                input_path, quality, lossless, method, recursive, output_folder
            )

            print("=" * 60)
            print(f"Summary: {success_count} images converted successfully")
            print(f"Total time: {total_time:.2f}s")

            if success_count > 0:
                print(f"Average time per image: {total_time/success_count:.2f}s")

            if converter.failed_count > 0:
                print(f"Failed conversions: {converter.failed_count}")

        else:
            print(f"Error: Input is neither a file nor directory: {input_path}")
            sys.exit(1)

    except KeyboardInterrupt:
        print("\nOperation cancelled by user.")
        sys.exit(1)
    except Exception as e:
        print(f"Unexpected error: {str(e)}")
        sys.exit(1)


if __name__ == "__main__":
    main()