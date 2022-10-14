use serde::Deserialize;
use serde_json::Value;

use super::related_streams::RelatedStream;
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Playlist {
    banner_url: String, // The banner of the playlist
    name: String, // The name of the playlist
    nextpage: String, // A JSON encoded page, which is used for the nextpage endpoint.
    related_streams: Vec<RelatedStream>, // A list of videos from the playlist
    thumbnail_url: String, // The thumbnail of the playlist
    uploader: String, // The name of the creator of the playlist
    uploader_svatar: String, // The avatar of the creator of the playlist
    uploader_url: String, // The URL of the creator of the playlist
    videos: i32 // The number of videos in the playlist
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistNext {
    nextpage: String,
    related_streams: Vec<RelatedStream>,
}

impl From<Value> for Playlist{
    fn from(value: Value) -> Self {
        serde_json::from_value(value).unwrap()
    }
}
impl From<Value> for PlaylistNext{
    fn from(value: Value) -> Self {
        serde_json::from_value(value).unwrap()
    }
}