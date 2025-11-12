use rbook::{prelude::*, Ebook, Epub};
use std::path::{Path, PathBuf};
use tokio::fs;
use walkdir::WalkDir;

// A struct to hold metadata parsed from an EPUB file.
pub struct BookMetadata {
    pub title: String,
    pub authors: Vec<String>,
    pub published_date: Option<String>,
    pub publishers: Vec<String>,
    pub isbn: Option<String>,
    pub file_path: String,
    pub cover_data: Option<(Vec<u8>, String)>, // (data, mime_type)
}

// TODO: Test this function
/// Scans for epub files to be added to the library
pub fn scan_epubs<P: AsRef<Path> + Send + 'static>(dir: P) -> Vec<PathBuf> {
    let walker = WalkDir::new(dir).into_iter();
    // collect all .epub files in the directory
    walker
        .filter_map(Result::ok) // Filter out entries that resulted in an error
        .filter(|e| e.file_type().is_file()) // Filter to include only files
        .map(|e| e.into_path()) // Get the path of each entry
        .filter(|p| {
            // Filter to include only .epub files
            p.extension()
                .and_then(|s| s.to_str())
                .map(|ext| ext.eq_ignore_ascii_case("epub"))
                .unwrap_or(false)
        })
        .collect() // Collect the filtered paths into a vector
}
//TODO: Test this function
/// Parses metadata from an EPUB file and returns a `BookMetadata` struct.
pub fn parse_epub_meta(
    path: String,
) -> Result<BookMetadata, Box<dyn std::error::Error + Send + Sync>> {
    let book = Epub::open(&path)?;
    let metadata = book.metadata();

    let title = metadata
        .title()
        .map(|t| t.value().to_string())
        .unwrap_or_else(|| "Unknown Title".to_string());

    let mut authors: Vec<String> = metadata.creators().map(|c| c.value().to_string()).collect();

    let mut publishers: Vec<String> = metadata
        .publishers()
        .map(|p| p.value().to_string())
        .collect::<Vec<String>>();

    if publishers.is_empty() {
        publishers.push("Unknown Publisher".to_string());
    }

    if authors.is_empty() {
        authors.push("Unknown Author".to_string());
    }

    let published_date = metadata.publication_date().map(|d| d.to_string());

    let isbn = metadata
        .identifiers()
        .find(|i| i.value().starts_with("urn:isbn:"))
        .map(|i| i.value().to_string());

    let cover_data = if let Some(cover_image) = book.manifest().images().next() {
        let mime_type = cover_image.media_type().to_string();
        cover_image
            .read_bytes()
            .ok()
            .map(|bytes| (bytes, mime_type))
    } else {
        None
    };

    Ok(BookMetadata {
        title,
        authors,
        publishers,
        published_date,
        isbn,
        file_path: path,
        cover_data,
    })
}

/// Stores a cover image to disk and returns the path.
/// The cover is stored in a `covers` subdirectory of the current working directory.
pub async fn store_cover_to_disk(
    cover_data: &[u8],
    media_type: &str,
    base_filename: &str,
) -> Result<String, std::io::Error> {
    let extension = match media_type {
        "image/jpeg" => "jpg",
        "image/png" => "png",
        "image/gif" => "gif",
        _ => "jpg", // default to jpg
    };

    let sanitized_filename = sanitize_filename(base_filename);
    let filename = format!("{}.{}", sanitized_filename, extension);

    let cover_dir = PathBuf::from("covers");
    fs::create_dir_all(&cover_dir).await?;

    let cover_path = cover_dir.join(&filename);
    fs::write(&cover_path, cover_data).await?;

    Ok(cover_path.to_string_lossy().to_string())
}

fn sanitize_filename(filename: &str) -> String {
    filename
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '.' || *c == '-' || *c == '_')
        .collect()
}
