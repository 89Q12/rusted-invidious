use serde::Deserialize;
use serde_json::Value;

use super::misc::{PlaylistVideo, VideoThumbnail};

#[derive(Deserialize)]
pub struct ChannelVideo{
    title: String,
    video_id: String,
    author_id: String,
    author_url: String,
    video_thumbnails: Vec<VideoThumbnail>,
    description: String,
    description_html: String,
    view_count: i64,
    published: i64,
    published_text: String,
    length_seconds: i64,
    paid: bool,
    premium: bool
}
#[derive(Deserialize)]
pub struct ChannelPlaylist{
    title: String,
    playlist_id: String,
    author: String,
    author_id: String,
    author_url: String,
    video_count: i32,
    videos:Vec<PlaylistVideo>
}
#[derive(Deserialize)]
pub struct ChannelPlaylists{
    continuation: Option<String>,
    playlists: Vec<ChannelPlaylist>
}

#[derive(Deserialize)]
pub struct ChannelVideos{
    videos: Vec<ChannelVideo>
}

impl From<Value> for ChannelPlaylists{
    fn from(value: Value) -> Self {
        serde_json::from_value(value).unwrap()
    }
}

impl From<Value> for ChannelVideos{
    fn from(value: Value) -> Self {
        let videos: Vec<ChannelVideo> = serde_json::from_value(value).unwrap();
        Self{
            videos
        }
    }
}