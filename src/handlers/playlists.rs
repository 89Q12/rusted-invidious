use std::{sync::Arc, collections::HashMap};
use askama::langid;
use tokio::sync::Mutex;
use axum::{Extension,extract::Query, response::Response, http::Request, body::Body};
use youtubei_rs::query::browse_id;
use crate::{config::State, structs::template_context::TemplateContext};
use askama::Template;
askama::localization!(LOCALES);
use super::{templates::view_playlist::ViewPlaylist, utils::render};

/// Handler for the /search path.
pub async fn view_playlist(Extension(state): Extension<Arc<Mutex<State>>>,Query(params): Query<HashMap<String, String>>,request: Request<Body>) -> Response {
    let lock = state.lock().await;
    let browse_result = browse_id("VL".to_owned() + &params.get("list").unwrap().to_string(), "".to_string(), &lock.yt_client_config).await.unwrap();
    let header = match browse_result.header.unwrap(){
        youtubei_rs::types::enums::HeaderContents::PlaylistHeaderRenderer(pl_header) => pl_header,
        _ => unreachable!(),
    };
    let playlist = ViewPlaylist{
        loc: askama::Locale::new(langid!("en-US"), &LOCALES),
        context: TemplateContext::new(&request, None, &lock.config),
        pl: crate::structs::Playlist::Playlist {
            contents: match browse_result.contents.unwrap(){                
                youtubei_rs::types::enums::TwoColumnTypes::TwoColumnBrowseResultsRenderer(brr) => match brr.tabs.get(0).unwrap().tab_renderer.as_ref().unwrap().content.as_ref().unwrap(){
                    youtubei_rs::types::enums::TabRendererContent::SectionListRenderer(slr) => match slr.contents.get(0).unwrap(){
                        youtubei_rs::types::enums::ItemSectionRendererContents::ItemSectionRenderer(isc) =>match isc.contents.get(0).unwrap(){
                                youtubei_rs::types::enums::ItemSectionRendererContents::PlaylistVideoListRenderer(playlist) => playlist.contents.clone(),
                                _ => unreachable!(),
                            },
                            _ => unreachable!(),
                        }
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
            },
            playlist_id: header.playlist_id,
            title: header.title,
            play_endpoint: header.play_endpoint.watch_endpoint.unwrap(),
            num_videos_text: header.num_videos_text,
            description_text: header.description_text,
            owner_text: header.owner_text,
            view_count_text: header.view_count_text,
            privacy: header.privacy,
            owner_endpoint: header.owner_endpoint.browse_endpoint.unwrap(),
            stats: header.stats,
            brief_stats: header.brief_stats,
        }
    };
    render(playlist.render())
}