use std::path::{Path, PathBuf};
use uuid::Uuid;
use crate::configs::*;
use crate::requests::VideoQuality;

pub fn get_anime_dir_path(anime_id: &Uuid) -> PathBuf {
    let input_path = get_anime_dir_path_scheme()
        .replace("{anime_id}", &anime_id.to_string());
    Path::new(&input_path).into()
}

pub fn get_episode_dir_path (anime_id: &Uuid, episode_order: &usize) -> PathBuf {
    let input_path = get_episode_dir_path_scheme()
        .replace("{anime_id}", &anime_id.to_string())
        .replace("{episode_order}", &episode_order.to_string());
    Path::new(&input_path).into()
}
pub fn get_video_quality_dir_path (anime_id: &Uuid, episode_order: &usize, video_quality: &VideoQuality) -> PathBuf {
    let input_path = get_video_quality_dir_path_scheme()
        .replace("{anime_id}", &anime_id.to_string())
        .replace("{episode_order}", &episode_order.to_string())
        .replace("{video_quality}", &video_quality.as_str());
    Path::new(&input_path).into()
}
pub fn get_audio_dub_dir_path (anime_id: &Uuid, episode_order: &usize, dub_studio_id:&Uuid) -> PathBuf {
    let input_path = get_audio_dub_dir_path_scheme()
        .replace("{anime_id}", &anime_id.to_string())
        .replace("{episode_order}", &episode_order.to_string())
        .replace("{dub_studio_id}", &dub_studio_id.to_string());
    Path::new(&input_path).into()
}
pub fn get_sub_dir_path (anime_id: &Uuid, episode_order: &usize ) -> PathBuf {
    let input_path = get_sub_dir_path_scheme()
        .replace("{anime_id}", &anime_id.to_string())
        .replace("{episode_order}", &episode_order.to_string());
    Path::new(&input_path).into()
}
pub fn get_vtt_dir_path (anime_id: &Uuid, episode_order: &usize) -> PathBuf {
    let input_path = get_vtt_dir_path_scheme()
        .replace("{anime_id}", &anime_id.to_string())
        .replace("{episode_order}", &episode_order.to_string());
    Path::new(&input_path).into()
}