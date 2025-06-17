mod requests;
mod routes;
mod utils;
mod configs;
mod services;
mod errors;
mod constants;

use actix_web::{App, HttpServer, web};
use routes::*;
use crate::configs::{init_env_constants, init_path_schema_constants};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_env_constants();
    init_path_schema_constants();
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api/download")
                    .route("/anime_dir/{anime_id}/{file_name}/{extension}",
                           web::get().to(download_file_from_anime_dir)
                    )
                    .route(
                        "/anime_dir/{anime_id}/episodes/{episode_order}/image/{file_name}/{extension}",
                        web::get().to(download_image_file_from_episode_dir),
                    )
                    .route(
                        "/anime_dir/{anime_id}/episodes/{episode_order}/m3u8",
                        web::get().to(download_m3u8_file_from_episode_dir),
                    )
                    .route(
                        "/anime_dir/{anime_id}/episodes/{episode_order}/video/{video_quality}/ts/{ts_file_index}",
                        web::get().to(download_ts_file_from_video_dir),
                    )
                    .route(
                        "/anime_dir/{anime_id}/episodes/{episode_order}/video/{video_quality}/m3u8",
                        web::get().to(download_m3u8_file_from_video_dir),
                    )
                    .route(
                        "/anime_dir/{anime_id}/episodes/{episode_order}/audio/{dub_studio_id}/ts/{ts_file_index}",
                        web::get().to(download_ts_file_from_audio_dub_dir),
                    )
                    .route(
                        "/anime_dir/{anime_id}/episodes/{episode_order}/audio/{dub_studio_id}/m3u8",
                        web::get().to(download_m3u8_file_from_audio_dub_dir),
                    )
                    .route(
                        "/anime_dir/{anime_id}/episodes/{episode_order}/sub/lang/{language_code}",
                        web::get().to(download_lang_from_sub_dir),
                    )
                    .route(
                        "/anime_dir/{anime_id}/episodes/{episode_order}/suns/subs",
                        web::get().to(download_subs_file_from_sub_dir),
                    )
                    .route(
                        "/anime_dir/{anime_id}/episodes/{episode_order}/vtt/f/thumb/{extension}",
                        web::get().to(download_vtt_image_file_from_vtt_dir),
                    )
                    .route(
                        "/anime_dir/{anime_id}/episodes/{episode_order}/vtt/thumbnails",
                        web::get().to(download_vtt_file_from_vtt_dir),
                    )
            )
            .service(
                web::scope("/api/upload")
                    .route(
                        "/anime_dir/{anime_id}/{file_name}",
                        web::post().to(upload_file_to_anime_dir),
                    )
                    .route(
                        "/anime_dir/{anime_id}/episodes/{episode_order}/{file_name}",
                        web::post().to(upload_file_to_episode_dir),
                    )
                    .route(
                        "/anime_dir/{anime_id}/episodes/{episode_order}/video/{video_quality}/ts",
                        web::post().to(upload_ts_files_to_video_dir),
                    )
                    .route(
                        "/anime_dir/{anime_id}/episodes/{episode_order}/video/{video_quality}/m3u8",
                        web::post().to(upload_m3u8_file_to_video_dir),
                    )
                    .route(
                        "/anime_dir/{anime_id}/episodes/{episode_order}/audio/{dub_studio_id}/ts",
                        web::post().to(upload_ts_files_to_audio_dub_dir),
                    )
                    .route(
                        "/anime_dir/{anime_id}/episodes/{episode_order}/audio/{dub_studio_id}/m3u8",
                        web::post().to(upload_m3u8_file_to_audio_dub_dir),
                    )
                    .route(
                        "/anime_dir/{anime_id}/episodes/{episode_order}/sub/{language_code}",
                        web::post().to(upload_lang_file_to_sub_dir),
                    )
                    .route(
                        "/anime_dir/{anime_id}/episodes/{episode_order}/sub/txt",
                        web::post().to(upload_subs_file_to_sub_dir),
                    )
                    .route(
                        "/anime_dir/{anime_id}/episodes/{episode_order}/vtt/{file_name}",
                        web::post().to(upload_file_to_to_vtt_dir),
                    )
            )
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
