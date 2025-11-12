use std::path::Path;
use walkdir::WalkDir;

use crate::data::models::books::Books;

// TODO: Test this function
pub async fn scan_epubs<P: AsRef<Path> + Send + 'static>(dir: P) {
    let mut results = Vec::new();
    let walker = WalkDir::new(dir).into_iter();
    // TODO: Refactor this
    for entry in walker.filter_map(Result::ok) {
        if entry.path().is_file() {
            if let Some(ext) = entry.path().extension() {
                if ext == "epub" {
                    results.push(entry.path().to_path_buf());
                }
            }
        }
    }
}

// TODO: Parse metadata from found EPUB files and store in database
pub async fn parse_epub(path: &str) -> Result<Books, Box<dyn std::error::Error>> {
    // Placeholder for future implementation
    // Logic for parsing EPUB metadata and returning a Books instance to prepare for database insertion
    // Should use rbook's EpubMetadata for parsing
    // Refer to src/data/models/books.rs for the Books struct definition
    unimplemented!()
}

pub async fn store_cover_to_disk() -> Result<(), Box<dyn std::error::Error>> {
    // Placeholder for future implementation
    // Should use the cover_image_bytes function to get the image bytes and store it to disk
    unimplemented!()
}

async fn cover_image_bytes() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Placeholder for future implementation
    // Refer to rbook's documentation for extracting cover image bytes from an EPUB file
    unimplemented!()
}
