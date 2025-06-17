use actix_files::NamedFile;
use actix_web::{HttpRequest, HttpResponse};
use crate::errors::DownloadError;

pub fn handle_download_result(req: &HttpRequest, result: Result<NamedFile, DownloadError>) -> HttpResponse {
    match result {
        Ok(named_file) => {
            named_file.into_response(req)
        },
        Err(DownloadError::FileNotFound) => HttpResponse::NotFound().json(serde_json::json!({
            "status": "error",
            "error": "file_not_found",
            "message": "File not found"
        })),
        Err(DownloadError::NotAFile) => HttpResponse::BadRequest().json(serde_json::json!({
            "status": "error",
            "error": "not_a_file",
            "message": "Path is not a file"
        })),
        Err(DownloadError::ExtensionNotFound) => HttpResponse::BadRequest().json(serde_json::json!({
            "status": "error",
            "error": "extension_not_found",
            "message": "File extension not found"
        })),
        Err(DownloadError::ExtensionNotAllowed) => HttpResponse::BadRequest().json(serde_json::json!({
            "status": "error",
            "error": "extension_not_allowed",
            "message": "File extension is not allowed"
        })),
        Err(DownloadError::InvalidStreamingFormat) => HttpResponse::BadRequest().json(serde_json::json!({
            "status": "error",
            "error": "invalid_streaming_format",
            "message": "Invalid streaming file format"
        })),
        Err(DownloadError::InvalidSubtitleFormat) => HttpResponse::BadRequest().json(serde_json::json!({
            "status": "error",
            "error": "invalid_subtitle_format",
            "message": "Invalid subtitle file format"
        })),
        Err(e) => {
            eprintln!("Internal server error: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "error",
                "error": "internal_error",
                "message": "Internal server error"
            }))
        }
    }
}