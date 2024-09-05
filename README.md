
# Rust Photo Organizer with ARW to JPEG Conversion

## Overview

This is a Rust-based command-line tool that organizes photos by date. The tool reads ARW (Sony RAW) files, extracts the date the photo was taken using `exiftool`, and moves the photo to a folder named by the date. Optionally, the tool can also convert ARW files to JPEG using `dcraw`, then compress the resulting JPEG using **ImageMagick** or **jpegoptim**.

## Features

- Organizes ARW files into folders based on the date the photo was taken.
- Converts ARW files to JPEG using `dcraw`.
- Compresses the generated JPEG files using **ImageMagick** (`convert`) or **jpegoptim`.
- Allows specifying JPEG quality via command-line arguments.

## Requirements

Before using the tool, ensure that the following are installed:

1. **Rust**: You need the Rust toolchain. Install it from [here](https://www.rust-lang.org/tools/install).
2. **dcraw**: To handle the ARW to JPEG conversion.
    - **Linux/macOS**: Install via `sudo apt install dcraw` or `brew install dcraw`.
    - **Windows**: [Download dcraw](https://www.cybercom.net/~dcoffin/dcraw/).
3. **ImageMagick** or **jpegoptim**: For JPEG compression.
    - **ImageMagick**:
      - **Linux/macOS**: Install via `sudo apt install imagemagick` or `brew install imagemagick`.
    - **jpegoptim**:
      - **Linux/macOS**: Install via `sudo apt install jpegoptim` or `brew install jpegoptim`.

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/Nyovelt/RAW-Organizer.git
   cd photo_organizer
   ```

2. Build the project using Cargo:
   ```bash
   cargo build --release
   ```

## Usage

The program requires a source directory containing ARW files and a destination directory where the organized photos will be stored.

```bash
cargo run <source_directory> <destination_directory> [--convert-to-jpg <quality:0-100>]
```

### Example Usages

1. **Organize ARW files into date folders without JPEG conversion**:
   ```bash
   cargo run /path/to/raw_photos /path/to/organized_photos
   ```

2. **Convert ARW files to JPEG and organize them (default quality of 80)**:
   ```bash
   cargo run /path/to/raw_photos /path/to/organized_photos --convert-to-jpg
   ```

3. **Convert ARW files to JPEG with custom quality (e.g., 90)**:
   ```bash
   cargo run /path/to/raw_photos /path/to/organized_photos --convert-to-jpg 90
   ```

### Command-Line Arguments

- **`<source_directory>`**: The directory where the ARW files are stored.
- **`<destination_directory>`**: The directory where organized photos will be moved.
- **`--convert-to-jpg [quality]`**: Optional flag to convert ARW files to JPEG. You can specify the JPEG quality (0-100) with an optional argument, where lower values give higher compression.

## Workflow

1. **Date Extraction**: The tool uses `exiftool` to extract the date a photo was taken (`DateTimeOriginal`).
2. **File Organization**: ARW files are moved to date-based folders, named in the format `YYYY-MM-DD`.
3. **JPEG Conversion** (Optional):
   - `dcraw` is used to convert ARW files to JPEG.
4. **JPEG Compression** (Optional):
   - The resulting JPEG files can be compressed using either **ImageMagick's** `convert` tool or **jpegoptim**.

## Dependencies

The following dependencies are required for the project:

- **`chrono`**: For working with dates.
- **`std::process::Command`**: To execute external commands like `dcraw`, `convert`, or `jpegoptim`.
- **`exiftool`**: For extracting metadata from the ARW files.

## Troubleshooting

- Ensure that **`dcraw`**, **`convert`** (ImageMagick), or **`jpegoptim`** are installed and accessible in your system's PATH.
- Make sure the JPEG quality is specified as an integer between `0` (lowest quality, highest compression) and `100` (best quality, lowest compression).


