use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use chrono::NaiveDate; // For working with dates

// Function to get photo's date taken using exiftool
fn get_photo_date_taken(photo_path: &Path) -> io::Result<Option<NaiveDate>> {
    let output = Command::new("exiftool")
        .arg("-DateTimeOriginal")
        .arg("-s3")
        .arg(photo_path)
        .output()?;

    if output.status.success() {
        let date_str = String::from_utf8_lossy(&output.stdout).trim().to_string();

        if !date_str.is_empty() {
            if let Ok(date) = NaiveDate::parse_from_str(&date_str[..10], "%Y:%m:%d") {
                return Ok(Some(date));
            }
        }
    }

    Ok(None)
}

// Function to compress JPEG using ImageMagick's convert or jpegoptim
fn compress_jpeg(jpeg_path: &Path, quality: u8) -> io::Result<()> {
    // First, attempt to use ImageMagick's convert
    let status_convert = Command::new("magick")
        .arg(jpeg_path)
        .arg("-quality")
        .arg(format!("{}", quality)) // Specify compression quality
        .arg(jpeg_path) // Overwrite the existing file
        .status();
    
    if let Ok(status) = status_convert {
        if status.success() {
            println!("JPEG successfully compressed using convert.");
            return Ok(());
        }
    }

    // Fallback to jpegoptim if convert is not available
    let status_jpegoptim = Command::new("jpegoptim")
        .arg("--max")
        .arg(format!("{}", quality)) // Specify maximum quality
        .arg(jpeg_path)
        .status()?;

    if status_jpegoptim.success() {
        println!("JPEG successfully compressed using jpegoptim.");
    } else {
        eprintln!("Failed to compress JPEG at {:?}", jpeg_path);
    }

    Ok(())
}

// Function to convert ARW to JPEG using dcraw and compress it
fn convert_arw_to_jpeg(photo_path: &Path, output_path: &Path, quality: u8) -> io::Result<()> {
    // Use dcraw to convert ARW to JPEG format
    let convert_output = Command::new("dcraw")
        .arg("-c")  // Output to stdout
        .arg("-w")  // Use camera white balance
        .arg(photo_path)
        .output();

    if let Ok(output) = convert_output {
        if output.status.success() {
            // Write the converted JPEG to the output path
            fs::write(output_path, output.stdout)?;
            println!("JPEG generated for {:?}", photo_path.file_name().unwrap());

            // Compress the JPEG after it is generated
            compress_jpeg(output_path, quality)?;
        } else {
            eprintln!("Failed to convert {:?} to JPEG using dcraw.", photo_path.file_name().unwrap());
        }
    } else {
        eprintln!("dcraw is not available for converting {:?}", photo_path.file_name().unwrap());
    }

    Ok(())
}

// Function to move a file to the target folder and optionally convert it to JPEG
fn move_file_to_date_folder(photo_path: &Path, destination_dir: &Path, date: NaiveDate, convert_to_jpg: bool, jpeg_quality: u8) -> io::Result<()> {
    let date_folder = destination_dir.join(date.format("%Y-%m-%d").to_string());

    // Create the date folder if it doesn't exist
    if !date_folder.exists() {
        fs::create_dir_all(&date_folder)?;
    }

    let target_path = date_folder.join(photo_path.file_name().unwrap());
    fs::rename(photo_path, &target_path)?;

    // If the user requested JPEG conversion, convert the file
    if convert_to_jpg {
        let jpg_output_path = date_folder.join(photo_path.with_extension("jpg").file_name().unwrap());
        convert_arw_to_jpeg(&target_path, &jpg_output_path, jpeg_quality)?;
    }

    Ok(())
}

// Main function to organize photos
fn organize_photos_by_date(source_dir: &Path, destination_dir: &Path, convert_to_jpg: bool, jpeg_quality: u8) -> io::Result<()> {
    for entry in fs::read_dir(source_dir)? {
        let entry = entry?;
        let path = entry.path();

        // Only consider ARW files
        if path.extension().map_or(false, |ext| ext.eq_ignore_ascii_case("arw")) {
            if let Some(date) = get_photo_date_taken(&path)? {
                move_file_to_date_folder(&path, destination_dir, date, convert_to_jpg, jpeg_quality)?;
                println!("Moved file {:?} to folder for date {:?}", path.file_name().unwrap(), date);
            } else {
                println!("Could not determine the date for file {:?}", path.file_name().unwrap());
            }
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the required number of arguments is passed
    if args.len() < 3 {
        eprintln!("Usage: {} <source_directory> <destination_directory> [--convert-to-jpg <quality:0-100>]", args[0]);
        std::process::exit(1);
    }

    // Get the source and destination directories from arguments
    let source_directory = PathBuf::from(&args[1]);
    let destination_directory = PathBuf::from(&args[2]);

    // Check for the optional flag --convert-to-jpg and quality argument
    let convert_to_jpg = args.contains(&"--convert-to-jpg".to_string());
    let jpeg_quality = if convert_to_jpg {
        // Get quality value if provided, defaulting to 80 if not specified
        args.get(args.len() - 1).unwrap_or(&"80".to_string()).parse::<u8>().unwrap_or(80)
    } else {
        80 // Default quality for JPEG if not specified
    };

    // Organize the photos by date and optionally convert to JPG
    organize_photos_by_date(&source_directory, &destination_directory, convert_to_jpg, jpeg_quality)
}
