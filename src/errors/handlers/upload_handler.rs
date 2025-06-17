use actix_web::HttpResponse;

pub fn handle_upload_one_file_result(result: Result<(), UploadError>) -> HttpResponse {
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
        Err(UploadError::FilenameNotFound) => HttpResponse::BadRequest().json(serde_json::json!({
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
        // Обработка IO ошибок (если UploadError реализует From<std::io::Error>)
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
            "error": "internal_error",
            "message": format!("Internal server error: {}", e)
        }))
    }
}

// Обработчик ошибок для upload_ts_files_with_validation
pub fn handle_upload_ts_files_result(result: Result<Vec<String>, UploadError>) -> HttpResponse {
    match result {
        Ok(uploaded_files) => HttpResponse::Ok().json(serde_json::json!({
            "status": "success",
            "message": format!("Successfully uploaded {} files", uploaded_files.len()),
            "uploaded_files": uploaded_files
        })),
        Err(UploadError::MultipartError(msg)) => HttpResponse::BadRequest().json(serde_json::json!({
            "status": "error",
            "error": "multipart_error",
            "message": format!("Multipart processing error: {}", msg)
        })),
        Err(UploadError::FilenameNotFound) => HttpResponse::BadRequest().json(serde_json::json!({
            "status": "error",
            "error": "filename_not_found",
            "message": "Filename not found in multipart data"
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
        // Обработка остальных ошибок
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
            "error": "internal_error",
            "message": format!("Internal server error: {}", e)
        }))
    }
}