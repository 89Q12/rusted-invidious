use serde::Deserialize;
use serde_json::Value;

use super::misc::RelatedStream;
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Playlist {
    pub banner_url: String, // The banner of the playlist
    pub name: String, // The name of the playlist
    pub nextpage: String, // A JSON encoded page, which is used for the nextpage endpoint.
    pub related_streams: Vec<RelatedStream>, // A list of videos from the playlist
    pub thumbnail_url: String, // The thumbnail of the playlist
    pub uploader: String, // The name of the creator of the playlist
    pub uploader_svatar: String, // The avatar of the creator of the playlist
    pub uploader_url: String, // The URL of the creator of the playlist
    pub videos: i32 // The number of videos in the playlist
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchPlaylist {
    name: String, // The name of the playlist
    url: String, // url to visit
    thumbnail: String, // The thumbnail of the playlist
    uploader_name: String, // The name of the creator of the playlist
    videos: i32 // The number of videos in the playlist
}


impl From<Value> for Playlist{
    fn from(value: Value) -> Self {
        serde_json::from_value(value).unwrap()
    }
}
