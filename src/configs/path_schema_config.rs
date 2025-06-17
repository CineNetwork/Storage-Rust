use crate::configs::get_allowed_base_dir;
use std::sync::OnceLock;

static ANIME_DIR_PATH_SCHEME: OnceLock<String> = OnceLock::new();
static EPISODE_DIR_PATH_SCHEME: OnceLock<String> = OnceLock::new();
static VIDEO_QUALITY_DIR_PATH_SCHEME: OnceLock<String> = OnceLock::new();
static AUDIO_DUB_DIR_PATH_SCHEME: OnceLock<String> = OnceLock::new();
static SUB_DIR_PATH_SCHEME: OnceLock<String> = OnceLock::new();
static VTT_DIR_PATH_SCHEME: OnceLock<String> = OnceLock::new();
pub fn init_path_schema_constants() -> Result<(), Box<dyn std::error::Error>> {
    ANIME_DIR_PATH_SCHEME
        .set(get_allowed_base_dir().to_owned() + "/{anime_id}")
        .map_err(|_| "Failed to set ANIME_DIR_PATH_SCHEME")?;

    EPISODE_DIR_PATH_SCHEME
        .set(get_allowed_base_dir().to_owned() + "/{anime_id}/episodes/{episode_order}")
        .map_err(|_| "Failed to set EPISODE_DIR_PATH_SCHEME")?;

    VIDEO_QUALITY_DIR_PATH_SCHEME
        .set(
            get_allowed_base_dir().to_owned()
                + "/{anime_id}/episodes/{episode_order}/video/{video_quality}",
        )
        .map_err(|_| "Failed to set VIDEO_QUALITY_DIR_PATH_SCHEME")?;

    AUDIO_DUB_DIR_PATH_SCHEME
        .set(
            get_allowed_base_dir().to_owned()
                + "/{anime_id}/episodes/{episode_order}/audio/{dub_studio_id}",
        )
        .map_err(|_| "Failed to set AUDIO_DUB_DIR_PATH_SCHEME")?;

    SUB_DIR_PATH_SCHEME
        .set(get_allowed_base_dir().to_owned() + "/{anime_id}/episodes/{episode_order}/sub")
        .map_err(|_| "Failed to set SUB_DIR_PATH_SCHEME")?;

    VTT_DIR_PATH_SCHEME
        .set(get_allowed_base_dir().to_owned() + "/{anime_id}/episodes/{episode_order}/vtt")
        .map_err(|_| "Failed to set VTT_DIR_PATH_SCHEME")?;

    Ok(())
}

pub fn get_anime_dir_path_scheme() -> &'static str {
    ANIME_DIR_PATH_SCHEME
        .get()
        .expect("ANIME_DIR_PATH_SCHEME not initialized")
}

pub fn get_episode_dir_path_scheme() -> &'static str {
    EPISODE_DIR_PATH_SCHEME
        .get()
        .expect("EPISODE_DIR_PATH_SCHEME not initialized")
}

pub fn get_video_quality_dir_path_scheme() -> &'static str {
    VIDEO_QUALITY_DIR_PATH_SCHEME
        .get()
        .expect("VIDEO_QUALITY_DIR_PATH_SCHEME not initialized")
}

pub fn get_audio_dub_dir_path_scheme() -> &'static str {
    AUDIO_DUB_DIR_PATH_SCHEME
        .get()
        .expect("AUDIO_DUB_DIR_PATH_SCHEME not initialized")
}

pub fn get_sub_dir_path_scheme() -> &'static str {
    SUB_DIR_PATH_SCHEME
        .get()
        .expect("SUB_DIR_PATH_SCHEME not initialized")
}

pub fn get_vtt_dir_path_scheme() -> &'static str {
    VTT_DIR_PATH_SCHEME
        .get()
        .expect("VTT_DIR_PATH_SCHEME not initialized")
}
