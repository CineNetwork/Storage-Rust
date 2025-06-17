use std::env;
use std::fmt::format;
use std::sync::OnceLock;
use dotenv::dotenv;

// OnceLock для ленивой инициализации
static ALLOWED_BASE_DIR: OnceLock<String> = OnceLock::new();
static MAX_FILE_SIZE: OnceLock<u64> = OnceLock::new();
// static ALLOWED_EXTENSIONS: OnceLock<Vec<String>> = OnceLock::new();
// ALLOWED_EXTENSIONS=vtt,jpg,jpeg,png,ts,txt,m3u8,gif
pub fn init_env_constants() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // Инициализируем константы
    ALLOWED_BASE_DIR.set(
        env::var("ALLOWED_BASE_DIR")?
    ).map_err(|_| "Failed to set ALLOWED_BASE_DIR")?;

    MAX_FILE_SIZE.set(
        env::var("MAX_FILE_SIZE")?.parse::<u64>()?
    ).map_err(|_| "Failed to set MAX_FILE_SIZE")?;

    // ALLOWED_EXTENSIONS.set(
    //     env::var("ALLOWED_EXTENSIONS")?
    //         .split(',')
    //         .map(|s| s.trim().to_string())
    //         .collect()
    // ).map_err(|_| "Failed to set ALLOWED_EXTENSIONS")?;
    Ok(())
}

// Геттеры для доступа к значениям
pub fn get_allowed_base_dir() -> &'static str {
    ALLOWED_BASE_DIR.get().expect("ALLOWED_BASE_DIR not initialized")
}

pub fn get_max_file_size() -> u64 {
    *MAX_FILE_SIZE.get().expect("MAX_FILE_SIZE not initialized")
}

// pub fn get_allowed_extensions() -> &'static [String] {
//     ALLOWED_EXTENSIONS.get().expect("ALLOWED_EXTENSIONS not initialized")
// }
