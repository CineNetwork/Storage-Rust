use crate::constants::{
    ALLOWED_IMAGE_EXTENSIONS, MASTER_PLAYLIST_EXTENSION, PLAYLIST, PLAYLIST_EXTENSION,
    SUB_LANG_EXTENSION, SUBS, SUBS_EXTENSION, THUMBNAILS_EXTENSION,
};
use crate::errors::handle_upload_result;
use crate::requests::*;
use crate::services::{upload_one_file, upload_ts_files};
use crate::utils::{
    get_anime_dir_path, get_audio_dub_dir_path, get_episode_dir_path, get_sub_dir_path,
    get_video_quality_dir_path, get_vtt_dir_path,
};
use actix_multipart::Multipart;
use actix_web::{HttpResponse, web};

pub async fn upload_file_to_anime_dir(
    path_vars: web::Path<UploadFileToAnimeDir>,
    payload: Multipart,
) -> HttpResponse {
    handle_upload_result(
        upload_one_file(
            &get_anime_dir_path(&path_vars.anime_id),
            payload,
            ALLOWED_IMAGE_EXTENSIONS,
            path_vars.file_name.as_str(),
        )
        .await,
    )
}

pub async fn upload_file_to_episode_dir(
    path_vars: web::Path<UploadFileToEpisodeDir>,
    payload: Multipart,
) -> HttpResponse {
    handle_upload_result(
        upload_one_file(
            &get_episode_dir_path(&path_vars.anime_id, &path_vars.episode_order),
            payload,
            if path_vars.file_name == EpisodeDirFileName::MasterPlaylist {
                &[MASTER_PLAYLIST_EXTENSION]
            } else {
                ALLOWED_IMAGE_EXTENSIONS
            },
            path_vars.file_name.as_str(),
        )
        .await,
    )
}
pub async fn upload_ts_files_to_video_dir(
    path_vars: web::Path<UploadFileToVideoQualityDir>,
    payload: Multipart,
) -> HttpResponse {
    handle_upload_result(
        upload_ts_files(
            &get_video_quality_dir_path(
                &path_vars.anime_id,
                &path_vars.episode_order,
                &path_vars.video_quality,
            ),
            payload,
            // TODO нету лимита на количество ts файлов
            Option::None,
            // TODO нету лимита на размер ts файлов
            Option::None,
        )
        .await,
    )
}
pub async fn upload_m3u8_file_to_video_dir(
    path_vars: web::Path<UploadFileToVideoQualityDir>,
    payload: Multipart,
) -> HttpResponse {
    handle_upload_result(
        upload_one_file(
            &get_video_quality_dir_path(
                &path_vars.anime_id,
                &path_vars.episode_order,
                &path_vars.video_quality,
            ),
            payload,
            &[PLAYLIST_EXTENSION],
            PLAYLIST,
        )
        .await,
    )
}
pub async fn upload_ts_files_to_audio_dub_dir(
    path_vars: web::Path<UploadFileToAudioDubDir>,
    payload: Multipart,
) -> HttpResponse {
    handle_upload_result(
        upload_ts_files(
            &get_audio_dub_dir_path(
                &path_vars.anime_id,
                &path_vars.episode_order,
                &path_vars.dub_studio_id,
            ),
            payload,
            // TODO нету лимита на количество ts файлов
            Option::None,
            // TODO нету лимита на размер ts файлов
            Option::None,
        )
        .await,
    )
}
pub async fn upload_m3u8_file_to_audio_dub_dir(
    path_vars: web::Path<UploadFileToAudioDubDir>,
    payload: Multipart,
) -> HttpResponse {
    handle_upload_result(
        upload_one_file(
            &get_audio_dub_dir_path(
                &path_vars.anime_id,
                &path_vars.episode_order,
                &path_vars.dub_studio_id,
            ),
            payload,
            &[PLAYLIST_EXTENSION],
            AudioDubDirFileName::Playlist.as_str(),
        )
        .await,
    )
}
pub async fn upload_lang_file_to_sub_dir(
    path_vars: web::Path<UploadLangFileToSubDir>,
    payload: Multipart,
) -> HttpResponse {
    handle_upload_result(
        upload_one_file(
            &get_sub_dir_path(&path_vars.anime_id, &path_vars.episode_order),
            payload,
            &[SUB_LANG_EXTENSION],
            &path_vars.language_code.to_string(),
        )
        .await,
    )
}
pub async fn upload_subs_file_to_sub_dir(
    path_vars: web::Path<UploadTextFileToSubDir>,
    payload: Multipart,
) -> HttpResponse {
    handle_upload_result(
        upload_one_file(
            &get_sub_dir_path(&path_vars.anime_id, &path_vars.episode_order),
            payload,
            &[SUBS_EXTENSION],
            SUBS,
        )
        .await,
    )
}

pub async fn upload_file_to_to_vtt_dir(
    path_vars: web::Path<UploadFileToVttDir>,
    payload: Multipart,
) -> HttpResponse {
    handle_upload_result(
        upload_one_file(
            &get_vtt_dir_path(&path_vars.anime_id, &path_vars.episode_order),
            payload,
            if path_vars.file_name == VttDirFileName::Thumbnails {
                &[THUMBNAILS_EXTENSION]
            } else {
                ALLOWED_IMAGE_EXTENSIONS
            },
            path_vars.file_name.as_str(),
        )
        .await,
    )
}
