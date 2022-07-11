use std::{sync::Arc, collections::HashMap, fmt::Debug};
use askama::{langid, Template};
use axum::{response::{Response, Redirect, IntoResponse, Html}, Extension,extract::{path::Path, Query}, http::{StatusCode, Request}, middleware::Next, body::Body};
use tokio::sync::Mutex;
use tracing::Level;
use youtubei_rs::{query::{player, next_video_id, resolve}, types::video::{VideoPrimaryInfoRenderer, VideoSecondaryInfoRenderer}, types::misc::Format};
use youtubei_rs::utils::*;
use crate::{config::State, structs::{Video::Video, player::Player}};
use super::{utils::{result_to_body, string_to_body, build_params, render}, templates::{base::Base, watch::Watch}};

askama::localization!(LOCALES);
/// Handler for the /watch?v=id path and renders the watch page
/// TODO clean up, reduce smell
pub async fn watch_v(Extension(state): Extension<Arc<Mutex<State>>>,Query(params): Query<HashMap<String, String>>,request: Request<Body>) -> Response {
    tracing::event!(target: "web_handlers", Level::DEBUG, "entering watch handler");
    let lock = state.lock().await;
    let player_call = player(params.get("v").unwrap().to_string(), "".to_string(), &lock.yt_client_config);
    let next_call = next_video_id(params.get("v").unwrap().to_string(), "".to_string(), &lock.yt_client_config);
    tracing::event!(target: "web_handlers", Level::TRACE, "Joining async calls");
    let (player_res, next_res) = tokio::join!(player_call, next_call);
    tracing::event!(target: "web_handlers", Level::TRACE, "Finished joining async calls");
    let mut player= match player_res {
        Ok(player) =>match player.playability_status.status.as_str(){
            // TODO: Better way to handle errors
            "ERROR" => return string_to_body(StatusCode::NOT_FOUND.to_string()),
            _ => player
        },
        Err(err) => match err {
            youtubei_rs::types::error::Errors::RequestError(_) => return string_to_body(StatusCode::NOT_FOUND.to_string()),
            youtubei_rs::types::error::Errors::ParseError(_) => {
                tracing::event!(target: "web_handlers", Level::DEBUG, "Error occurred while requesting a player data retrying wit android client");
                match player(params.get("v").unwrap().to_string(), "".to_string(), &lock.yt_client_config).await{
                    Ok(player) => player,
                    Err(_) => return string_to_body(StatusCode::NOT_FOUND.to_string()),
                }
            }
        },
    };
    drop(lock);
    let next= match next_res {
        Ok(next) => next,
        Err(err) => match err{
            youtubei_rs::types::error::Errors::RequestError(error) => {
                tracing::event!(target: "web_handlers", Level::ERROR, "Request failed to next endpoint.\n Error: {}\n Video ID: {}", error.to_string(),params.get("v").unwrap().to_string() );
                return string_to_body(StatusCode::NOT_FOUND.to_string())
            },
            youtubei_rs::types::error::Errors::ParseError(error) => {
                tracing::event!(target: "web_handlers", Level::ERROR, "Parsing failed on next endpoint data.\n Error: {}\n Video ID: {}", error.to_string(),params.get("v").unwrap().to_string() );
                return string_to_body(StatusCode::NOT_FOUND.to_string())
            },
        },
    };
    let related_videos = next.contents.as_ref().unwrap().two_column_watch_next_results.as_ref().unwrap().secondary_results.secondary_results.results.iter().filter_map(|item| {
        match item{
            youtubei_rs::types::misc::NextContents::CompactVideoRenderer(cmp) => Some(cmp.to_owned()),
            _ => return None,
        }
    }).collect::<Vec<_>>();
    let primary_video_renderer: &VideoPrimaryInfoRenderer = next.contents.as_ref().unwrap().two_column_watch_next_results.as_ref().unwrap().results.results.contents.iter().filter_map(|item| {
        return match item{
            youtubei_rs::types::misc::NextContents::VideoPrimaryInfoRenderer(vpr) =>Some(vpr),
            _ => None
        };
    }).collect::<Vec<_>>().get(0).unwrap();
    let secondary_video_renderer: &VideoSecondaryInfoRenderer = next.contents.as_ref().unwrap().two_column_watch_next_results.as_ref().unwrap().results.results.contents.iter().filter_map(|item| {
        return match item{
            youtubei_rs::types::misc::NextContents::VideoSecondaryInfoRenderer(vpr) =>Some(vpr),
            _ => None
        };
    }).collect::<Vec<_>>().get(0).unwrap();
    let comments_count: String = match next.contents.as_ref().unwrap().two_column_watch_next_results.as_ref().unwrap().results.results.contents.get(2) {
            Some(youtubei_rs::types::misc::NextContents::ItemSectionRenderer(comments)) => match comments.contents.get(0).unwrap(){
                youtubei_rs::types::misc::ItemSectionRendererContents::CommentsEntryPointHeaderRenderer(header) => header["commentCount"]["simpleText"].to_string(),
                _ => "".to_string()
            },
            _ => "".to_string(),
    };
    let author_verified = if let Some(badges) = &secondary_video_renderer.owner.video_owner_renderer.badges{
        get_author_verified(&&badges.get(0).unwrap().metadata_badge_renderer)
    }else{
        false
    };
    let mut category = String::from("");
    let license: Option<String> = if let Some(rows) = &secondary_video_renderer.metadata_row_container.metadata_row_container_renderer.rows{
        let mut return_str = None;
        for row in rows.iter(){
            let matched_row = match row{
                youtubei_rs::types::misc::MetadataRowContents::MetadataRowRenderer(row) => row,
                _ => unreachable!()
            };
            if matched_row.title.simple_text.as_str() == "License"{
                return_str = Some(matched_row.contents.get(0).unwrap().runs.as_ref().unwrap().get(0).unwrap().text.clone());
                break;
                
            };
            if matched_row.title.simple_text.as_str() == "Category"{
                category.push_str(&matched_row.clone().title.simple_text);
                return_str = None;
                break;
            }
        };
        return_str
    }else{
        None
    };
    let mut audio_streams: Vec<_> = player.streaming_data.formats.iter().filter(|format| format.mime_type.contains("audio")).collect();
    audio_streams.append(&mut player.streaming_data.adaptive_formats.iter().filter(|format| format.mime_type.contains("audio")).collect());
    let video = Video{
        thumbnail: player.video_details.thumbnail.thumbnails.last().unwrap().url.clone(),
        id: player.video_details.video_id,
        keywords: player.video_details.keywords.unwrap_or_default(),
        short_description: player.video_details.short_description,
        title: player.video_details.title,
        is_listed: player.video_details.is_private,
        reason: None,
        premiere_timestamp: None,
        live_now: player.video_details.is_live_content,
        views: player.video_details.view_count,
        likes: get_likes(&next),
        genre_url: None,
        genre: Some(category),
        license,
        ucid: player.video_details.channel_id.clone(),
        related_videos,
        allowed_regions: player.microformat.player_microformat_renderer.available_countries,
        author_thumbnail: get_owner_thumbnail(&next),
        author_id: player.video_details.channel_id.clone(),
        author: player.video_details.author,
        author_verified,
        sub_count_text: "".to_string(),
        description: get_description(&next),
        published: player.microformat.player_microformat_renderer.publish_date,
        length_seconds: player.video_details.length_seconds,
        engagement: "".to_string(), // Unsupported for now.
        wilson_score: "".to_string(),// Unsupported for now.
        rating: "".to_string(),// Unsupported for now.
        is_family_friendly: player.microformat.player_microformat_renderer.is_family_safe,
        projection_type: "".to_string(),// Unsupported for now.
        is_vr: false,// Unsupported for now.
        comments_count,
    };
    // Merge formats into one vec
    let mut formats =  player.streaming_data.formats.iter().filter(|format| !format.mime_type.contains("audio")).collect::<Vec<_>>();
    let mut adaptive_formats = player.streaming_data.adaptive_formats.iter().filter(|format| !format.mime_type.contains("audio")).collect::<Vec<_>>();
    formats.append(&mut adaptive_formats);
    let lock = state.lock().await;
    let captions = match player.captions{
        Some(captions) => captions.player_captions_tracklist_renderer.caption_tracks,
        None => Vec::with_capacity(0),
    };
    let preferred_captions = captions.iter().filter(|track|lock.preferences.captions.contains(&track.name.simple_text)).collect();


    let player_struct = Player{
        formats: &player.streaming_data.formats,
        audio_streams,
        captions: &captions,
        preferred_captions: preferred_captions,
        hls_manifest_url: None, // FIXME - should be  player.streaming_data.hls_manifest_url
        aspect_ratio: "16:9",
    };
    let base = Base{
        title: "invidious".to_string(),
        config: &lock.config,
        user: None,
        preferences: &lock.preferences,
        loc: askama::Locale::new(langid!("en-US"), &LOCALES),
        params: build_params(&request)
    };
    let watch = Watch{
        loc: askama::Locale::new(langid!("en-US"), &LOCALES),
        _parent: base,
        video,
        playlist: None,
        player: player_struct,
        comment_html: "".to_string(),
    };
    render(watch.render())
}

/// Handler for the paths /w/:id, /v/:id,/e/:id, /shorts/:id and /watch/:id it redirects to the /watch/v=id path
pub async fn redirect(Path(id): Path<String>) -> Redirect{
    Redirect::temporary(&("/watch?v=".to_owned()+ &id))
}

/// Handler for the path /watch_ajax which is used to mark a video as watched if the user is logged in
/// Protected route
pub async fn watch_ajax(Extension(state): Extension<Arc<Mutex<State>>>,body: String){
    todo!()
}

/// Handler for the paths /clip/:id it extracts the videoId from the clip id and redirects to the /watch?v=id path
pub async fn clip(Extension(state): Extension<Arc<Mutex<State>>>,Path(id): Path<String>) -> Redirect{
    let lock = state.lock().await;
    let video_id = resolve("https://www.youtube.com/clip/".to_owned()+ &id, &lock.yt_client_config).await;
    match video_id{
        Ok(vid) => Redirect::temporary(&("/watch?v=".to_owned() +&vid.endpoint.watch_endpoint.unwrap().video_id)),
        Err(_) => Redirect::temporary(&("/watch?v=".to_owned() + &id)),
    }
   
}
