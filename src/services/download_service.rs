use crate::errors::DownloadError;
use actix_files::NamedFile;
use std::path::Path;
pub fn get_named_file(path: &Path, extension: &str, filename: &str) -> Result<NamedFile, DownloadError> {
    let file_path = path.join(format!("{}.{}", filename, extension));
    if file_path.exists() && file_path.is_file() {
        return NamedFile::open(file_path).map_err(|_| DownloadError::FileNotFound);
    }
    Err(DownloadError::FileNotFound)
}
