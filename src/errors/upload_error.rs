#[derive(Debug)]
pub enum UploadError {
    MultipartError(String),
    FilenameNotFound,
    ExtensionNotFound,
    ExtensionNotAllowed,
    IoError(std::io::Error),
    ValidationError(String),
    InvalidFilename(String),
    InvalidExtension(String),
}

impl From<std::io::Error> for UploadError {
    fn from(error: std::io::Error) -> Self {
        UploadError::IoError(error)
    }
}