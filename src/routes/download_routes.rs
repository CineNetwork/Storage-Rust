use crate::constants::{MASTER_PLAYLIST, MASTER_PLAYLIST_EXTENSION, PLAYLIST, PLAYLIST_EXTENSION, SUBS, SUBS_EXTENSION, SUB_LANG_EXTENSION, THUMBNAILS_EXTENSION};
use crate::errors::handle_download_result;
use crate::requests::*;
use crate::services::get_named_file;
use crate::utils::{
    get_anime_dir_path, get_audio_dub_dir_path, get_episode_dir_path, get_sub_dir_path,
    get_video_quality_dir_path, get_vtt_dir_path,
};
use actix_web::{HttpRequest, HttpResponse, web};
use crate::requests::VttDirFileName::{Thumbnails, Thumbs};

pub async fn download_file_from_anime_dir(
    path_vars: web::Path<DownloadFileFromAnimeDir>,
    req: HttpRequest,
) -> HttpResponse {
    handle_download_result(
        &req,
        get_named_file(
            &get_anime_dir_path(&path_vars.anime_id),
            path_vars.extension.as_str(),
            path_vars.file_name.as_str(),
        ),
    )
}

pub async fn download_image_file_from_episode_dir(
    path_vars: web::Path<DownloadImageFileFromEpisodeDir>,
    req: HttpRequest,
) -> HttpResponse {
    handle_download_result(
        &req,
        get_named_file(
            &get_episode_dir_path(&path_vars.anime_id, &path_vars.episode_order),
            path_vars.extension.as_str(),
            path_vars.file_name.as_str(),
        ),
    )
}
pub async fn download_m3u8_file_from_episode_dir(
    path_vars: web::Path<DownloadM3U8FromEpisodeDir>,
    req: HttpRequest,
) -> HttpResponse {
    handle_download_result(
        &req,
        get_named_file(
            &get_episode_dir_path(&path_vars.anime_id, &path_vars.episode_order),
            MASTER_PLAYLIST_EXTENSION,
            MASTER_PLAYLIST,
        ),
    )
}

pub async fn download_ts_file_from_video_dir(
    path_vars: web::Path<DownloadTSFileFromVideoDir>,
    req: HttpRequest,
) -> HttpResponse {
    handle_download_result(
        &req,
        get_named_file(
            &get_video_quality_dir_path(
                &path_vars.anime_id,
                &path_vars.episode_order,
                &path_vars.video_quality,
            ),
            "ts",
            &path_vars.ts_file_index.to_string(),
        ),
    )
}

pub async fn download_m3u8_file_from_video_dir(
    path_vars: web::Path<DownloadM3u8FileFromVideoDir>,
    req: HttpRequest,
) -> HttpResponse {
    handle_download_result(
        &req,
        get_named_file(
            &get_video_quality_dir_path(
                &path_vars.anime_id,
                &path_vars.episode_order,
                &path_vars.video_quality,
            ),
            PLAYLIST_EXTENSION,
            PLAYLIST,
        ),
    )
}

pub async fn download_ts_file_from_audio_dub_dir(
    path_vars: web::Path<DownloadTsFileFromAudioDubDir>,
    req: HttpRequest,
) -> HttpResponse {
    handle_download_result(
        &req,
        get_named_file(
            &get_audio_dub_dir_path(
                &path_vars.anime_id,
                &path_vars.episode_order,
                &path_vars.dub_studio_id,
            ),
            "ts",
            &path_vars.ts_file_index.to_string(),
        ),
    )
}

pub async fn download_m3u8_file_from_audio_dub_dir(
    path_vars: web::Path<DownloadTsFileFromAudioDubDir>,
    req: HttpRequest,
) -> HttpResponse {
    handle_download_result(
        &req,
        get_named_file(
            &get_audio_dub_dir_path(
                &path_vars.anime_id,
                &path_vars.episode_order,
                &path_vars.dub_studio_id,
            ),
            PLAYLIST_EXTENSION,
            PLAYLIST,
        ),
    )
}

pub async fn download_lang_from_sub_dir(
    path_vars: web::Path<DownloadLangFileFromSubDir>,
    req: HttpRequest,
) -> HttpResponse {
    handle_download_result(
        &req,
        get_named_file(
            &get_sub_dir_path(
                &path_vars.anime_id,
                &path_vars.episode_order,
            ),
            SUB_LANG_EXTENSION,
            &path_vars.language_code.to_string(),
        ),
    )
}

pub async fn download_subs_file_from_sub_dir(path_vars: web::Path<DownloadSubsFileFromSubDir>, req: HttpRequest) -> HttpResponse {
    handle_download_result(
        &req,
        get_named_file(
            &get_sub_dir_path(
                &path_vars.anime_id,
                &path_vars.episode_order,
            ),
            SUBS_EXTENSION,
            SUBS,
        ),
    )
}

pub async fn download_vtt_image_file_from_vtt_dir(
    path_vars: web::Path<DownloadVttImageFileFromVttDir>,
    req: HttpRequest,
) -> HttpResponse {
    handle_download_result(
        &req,
        get_named_file(
            &get_vtt_dir_path(
                &path_vars.anime_id,
                &path_vars.episode_order,
            ),
            &path_vars.extension.as_str(),
            Thumbs.as_str(),
        ),
    )
}
pub async fn download_vtt_file_from_vtt_dir(
    path_vars: web::Path<DownloadVttFileFromVttDir>,
    req: HttpRequest,
) -> HttpResponse {
    handle_download_result(
        &req,
        get_named_file(
            &get_vtt_dir_path(
                &path_vars.anime_id,
                &path_vars.episode_order,
            ),
            THUMBNAILS_EXTENSION,
            Thumbnails.as_str(),
        ),
    )
}
