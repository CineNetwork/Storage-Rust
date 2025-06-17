use crate::requests::enums::{AnimeDirFileName, VideoQuality, VttDirFileName};
use codes_iso_639::part_1::LanguageCode;
use serde::Deserialize;
use uuid::Uuid;
use crate::requests::EpisodeDirFileName;

#[derive(Deserialize)]
pub struct UploadFileToAnimeDir {
    pub anime_id: Uuid,
    pub file_name: AnimeDirFileName,
}
#[derive(Deserialize)]
pub struct UploadFileToEpisodeDir {
    pub anime_id: Uuid,
    pub episode_order: usize,
    pub file_name: EpisodeDirFileName,
}
#[derive(Deserialize)]
pub struct UploadFileToVideoQualityDir {
    pub anime_id: Uuid,
    pub episode_order: usize,
    pub video_quality: VideoQuality,
}
#[derive(Deserialize)]
pub struct UploadFileToAudioDubDir {
    pub anime_id: Uuid,
    pub episode_order: usize,
    pub dub_studio_id: Uuid,
}

#[derive(Deserialize)]
pub struct UploadLangFileToSubDir {
    pub anime_id: Uuid,
    pub episode_order: usize,
    pub language_code: LanguageCode,
}
#[derive(Deserialize)]
pub struct UploadTextFileToSubDir {
    pub anime_id: Uuid,
    pub episode_order: usize,
}

#[derive(Deserialize)]
pub struct UploadFileToVttDir {
    pub anime_id: Uuid,
    pub episode_order: usize,
    pub file_name: VttDirFileName,
}
