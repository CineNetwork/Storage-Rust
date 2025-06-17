use crate::requests::enums::VideoQuality;
use codes_iso_639::part_1::LanguageCode;
use serde::Deserialize;
use uuid::Uuid;
use crate::requests::{AnimeDirFileName, EpisodeDirFileName, EpisodeDirImageFileName, ImageExtension, VttDirFileName};

#[derive(Deserialize)]
pub struct DownloadFileFromAnimeDir {
    pub anime_id: Uuid,
    pub file_name: AnimeDirFileName,
    pub extension: ImageExtension,
}

#[derive(Deserialize)]
pub struct DownloadImageFileFromEpisodeDir {
    pub anime_id: Uuid,
    pub episode_order: usize,
    pub file_name: EpisodeDirImageFileName,
    pub extension: ImageExtension,
}
#[derive(Deserialize)]
pub struct DownloadM3U8FromEpisodeDir {
    pub anime_id: Uuid,
    pub episode_order: usize,
}

#[derive(Deserialize)]
pub struct DownloadTSFileFromVideoDir {
    pub anime_id: Uuid,
    pub episode_order: usize,
    pub video_quality: VideoQuality,
    pub ts_file_index: usize,
}

#[derive(Deserialize)]
pub struct DownloadM3u8FileFromVideoDir {
    pub anime_id: Uuid,
    pub episode_order: usize,
    pub video_quality: VideoQuality,
}

#[derive(Deserialize)]
pub struct DownloadTsFileFromAudioDubDir {
    pub anime_id: Uuid,
    pub episode_order: usize,
    pub dub_studio_id: Uuid,
    pub ts_file_index: usize,
}

#[derive(Deserialize)]
pub struct DownloadM3u8FileFromAudioDubDir {
    pub anime_id: Uuid,
    pub episode_order: usize,
    pub dub_studio_id: Uuid,
}

#[derive(Deserialize)]
pub struct DownloadLangFileFromSubDir {
    pub anime_id: Uuid,
    pub episode_order: usize,
    pub language_code: LanguageCode,
}

#[derive(Deserialize)]
pub struct DownloadSubsFileFromSubDir {
    pub anime_id: Uuid,
    pub episode_order: usize,
}

#[derive(Deserialize)]
pub struct DownloadVttImageFileFromVttDir {
    pub anime_id: Uuid,
    pub episode_order: usize,
    pub extension: ImageExtension,
}
#[derive(Deserialize)]
pub struct DownloadVttFileFromVttDir {
    pub anime_id: Uuid,
    pub episode_order: usize,
}