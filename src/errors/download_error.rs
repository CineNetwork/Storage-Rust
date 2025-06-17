#[derive(Debug)]
pub enum DownloadError {
    FileNotFound,
    NotAFile,
    ExtensionNotFound,
    ExtensionNotAllowed,
    IoError(std::io::Error),
    InvalidStreamingFormat,
    InvalidSubtitleFormat,
}

impl From<std::io::Error> for DownloadError {
    fn from(error: std::io::Error) -> Self {
        DownloadError::IoError(error)
    }
}