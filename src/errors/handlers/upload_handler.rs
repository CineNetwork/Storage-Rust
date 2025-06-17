use actix_web::HttpResponse;
use crate::errors::UploadError;

pub fn handle_upload_result(result: Result<(), UploadError>) -> HttpResponse {
    match result {
        Ok(()) => HttpResponse::Ok().json(serde_json::json!({
            "status": "success",
            "message": "File uploaded successfully"
        })),
        Err(UploadError::MultipartError(msg)) => HttpResponse::BadRequest().json(serde_json::json!({
            "status": "error",
            "error": "multipart_error",
            "message": format!("Multipart processing error: {}", msg)
        })),
        Err(UploadError::FilenameNotFound) => HttpResponse::NotFound().json(serde_json::json!({
            "status": "error",
            "error": "filename_not_found",
            "message": "Filename not found in multipart data"
        })),
        Err(UploadError::ExtensionNotFound) => HttpResponse::BadRequest().json(serde_json::json!({
            "status": "error",
            "error": "extension_not_found",
            "message": "File extension not found"
        })),
        Err(UploadError::ExtensionNotAllowed) => HttpResponse::BadRequest().json(serde_json::json!({
            "status": "error",
            "error": "extension_not_allowed",
            "message": "File extension is not allowed"
        })),
        Err(UploadError::ValidationError(msg)) => HttpResponse::BadRequest().json(serde_json::json!({
            "status": "error",
            "error": "validation_error",
            "message": msg
        })),
        Err(UploadError::InvalidExtension(msg)) => HttpResponse::BadRequest().json(serde_json::json!({
            "status": "error",
            "error": "invalid_extension",
            "message": msg
        })),
        Err(UploadError::InvalidFilename(msg)) => HttpResponse::BadRequest().json(serde_json::json!({
            "status": "error",
            "error": "invalid_filename",
            "message": msg
        })),
        // Обработка IO ошибок (если UploadError реализует From<std::io::Error>)
        Err(e) => {
            eprintln!("Internal server error: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
            "error": "internal_error",
            "message": "Internal server error"
        }))}
    }
}