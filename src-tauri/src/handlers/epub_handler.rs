use std::path::Path;
use walkdir::WalkDir;
// TODO: Test this function
pub async fn scan_epubs<P: AsRef<Path> + Send + 'static>(dir: P) {
    let mut results = Vec::new();
    let walker = WalkDir::new(dir).into_iter();

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

// TODO: Send scanned epubs to database or further processing
