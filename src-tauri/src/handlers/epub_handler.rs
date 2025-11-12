use std::path::Path;
use walkdir::WalkDir;

use crate::data::models::books::Books;

// TODO: Test this function
// Scans for epub files to be added to the library
pub async fn scan_epubs<P: AsRef<Path> + Send + 'static>(dir: P) -> Vec<std::path::PathBuf> {
    let walker = WalkDir::new(dir).into_iter();
    // collect all .epub files in the directory (case-insensitive)
    walker
        .filter_map(Result::ok)// Filter out entries that resulted in an error
        .filter(|e| e.file_type().is_file())// Filter to include only files
        .map(|e| e.into_path())// Get the path of each entry
        .filter(|p| { // Filter to include only .epub files (case-insensitive)
            p.extension()
                .and_then(|s| s.to_str())
                .map(|ext| ext.eq_ignore_ascii_case("epub"))
                .unwrap_or(false)
        })
        .collect() // Collect the filtered paths into a vector
}

// TODO: Parse metadata from found EPUB files and store in database
pub async fn parse_epub_meta(path: &str) -> Result<Books, Box<dyn std::error::Error>> {
    // Placeholder for future implementation
    // Logic for parsing EPUB metadata and returning a Books instance to prepare for database insertion
    // Should use rbook's EpubMetadata for parsing
    // Refer to src/data/models/books.rs for the Books struct definition
    unimplemented!()
}
// Stores the cover image to disk and returns the path
pub async fn store_cover_to_disk() -> Result<String, Box<dyn std::error::Error>> {
    // Placeholder for future implementation
    // Should use the cover_image_bytes function to get the image bytes and store it to disk
    unimplemented!()
}

async fn cover_image_bytes() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Placeholder for future implementation
    // Refer to rbook's documentation for extracting cover image bytes from an EPUB file
    unimplemented!()
}
