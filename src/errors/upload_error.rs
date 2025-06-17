#[derive(Debug)]
enum UploadError {
    MultipartError(String),
    FilenameNotFound,
    ExtensionNotFound,
    ExtensionNotAllowed,
    IoError(std::io::Error),
}

impl From<std::io::Error> for UploadError {
    fn from(error: std::io::Error) -> Self {
        UploadError::IoError(error)
    }
}