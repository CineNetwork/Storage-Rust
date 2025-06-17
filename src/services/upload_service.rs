use actix_multipart::Multipart;
use futures_util::TryStreamExt;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use crate::errors::UploadError;

pub async fn upload_one_file(
    path: &Path,
    mut payload: Multipart,
    allowed_extensions: &[&str],
    end_filename: &str,
) -> Result<(), UploadError> {
    let mut field = payload
        .try_next()
        .await
        .map_err(|e| UploadError::MultipartError(e.to_string()))?
        .ok_or(UploadError::MultipartError("Empty multipart payload".to_string()))?;

    let filename = field
        .content_disposition()
        .and_then(|cd| cd.get_filename())
        .ok_or(UploadError::FilenameNotFound)?;

    let extension = Path::new(filename)
        .extension()
        .and_then(|ext| ext.to_str())
        .ok_or(UploadError::ExtensionNotFound)?;

    if !allowed_extensions.contains(&extension) {
        return Err(UploadError::ExtensionNotAllowed);
    }

    let file_path = path.join(format!("{}.{}", end_filename, extension));
    let mut file = File::create(&file_path).await?;

    while let Some(chunk) = field
        .try_next()
        .await
        .map_err(|e| UploadError::MultipartError(e.to_string()))?
    {
        file.write_all(&chunk).await?;
    }

    file.flush().await?;
    Ok(())
}

pub async fn upload_ts_files(
    path: &Path,
    mut payload: Multipart,
    max_files: Option<usize>,
    max_file_size: Option<u64>,
) -> Result<(), UploadError> {
    let mut uploaded_files = Vec::new();
    let mut file_count = 0;

    while let Some(mut field) = payload
        .try_next()
        .await
        .map_err(|e| UploadError::MultipartError(e.to_string()))?
    {
        // Проверяем лимит количества файлов
        if let Some(max) = max_files {
            if file_count >= max {
                return Err(UploadError::ValidationError(format!(
                    "Too many files. Maximum allowed: {}",
                    max
                )));
            }
        }

        let filename = field
            .content_disposition()
            .and_then(|cd| cd.get_filename())
            .ok_or(UploadError::FilenameNotFound)?;

        let file_index = validate_ts_filename(filename)?;
        let file_path = path.join(format!("{}.ts", file_index));

        if file_path.exists() {
            return Err(UploadError::ValidationError(format!(
                "File {}.ts already exists",
                file_index
            )));
        }

        let mut file = File::create(&file_path).await?;
        let mut bytes_written = 0u64;

        while let Some(chunk) = field
            .try_next()
            .await
            .map_err(|e| UploadError::MultipartError(e.to_string()))?
        {
            // Проверяем размер файла
            if let Some(max_size) = max_file_size {
                bytes_written += chunk.len() as u64;
                if bytes_written > max_size {
                    // Удаляем частично записанный файл
                    let _ = tokio::fs::remove_file(&file_path).await;
                    return Err(UploadError::ValidationError(format!(
                        "File {}.ts exceeds maximum size of {} bytes",
                        file_index,
                        max_size
                    )));
                }
            }

            file.write_all(&chunk).await?;
        }

        file.flush().await?;
        uploaded_files.push(format!("{}.ts", file_index));
        file_count += 1;
    }

    if uploaded_files.is_empty() {
        return Err(UploadError::ValidationError("No .ts files found in payload".to_string()));
    }

    Ok(())
}

fn validate_ts_filename(filename: &str) -> Result<usize, UploadError> {
    let path = Path::new(filename);

    // Проверяем расширение
    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .ok_or_else(|| UploadError::InvalidExtension("Missing file extension".to_string()))?;

    if extension != "ts" {
        return Err(UploadError::InvalidExtension(format!(
            "Expected .ts extension, got .{}",
            extension
        )));
    }

    // Проверяем имя файла (должно быть числом)
    let stem = path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .ok_or_else(|| UploadError::InvalidFilename("Invalid filename".to_string()))?;

    stem.parse::<usize>()
        .map_err(|_| UploadError::InvalidFilename(format!(
            "Filename '{}' is not a valid number",
            stem
        )))
}


