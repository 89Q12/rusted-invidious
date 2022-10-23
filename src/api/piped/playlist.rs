use serde::Deserialize;
use serde_json::Value;

use crate::api::{PlaylistTrait, PartialVideoTrait};
use crate::api::error::{ApiError, Errors};

use super::misc::RelatedStream;
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Playlist {
    pub banner_url: Option<String>, // The banner of the playlist
    pub name: String, // The name of the playlist
    pub nextpage: String, // A JSON encoded page, which is used for the nextpage endpoint.
    pub related_streams: Vec<RelatedStream>, // A list of videos from the playlist
    pub thumbnail_url: String, // The thumbnail of the playlist
    pub uploader: String, // The name of the creator of the playlist
    pub uploader_avatar: String, // The avatar of the creator of the playlist
    pub uploader_url: String, // The URL of the creator of the playlist
    pub videos: i32 // The number of videos in the playlist
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SearchPlaylist {
    pub name: String, // The name of the playlist
    pub url: String, // url to visit
    pub thumbnail: String, // The thumbnail of the playlist
    pub uploader_name: String, // The name of the creator of the playlist
    pub videos: i32 // The number of videos in the playlist
}

impl TryFrom<Value> for SearchPlaylist{
    type Error = ApiError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match serde_json::from_value(value){
            Ok(val) => Ok(val),
            Err(err) => Err(ApiError::new(Errors::RequestError, err.to_string())),
        }
    }
}

impl TryFrom<Value> for Playlist{
    type Error = ApiError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match serde_json::from_value(value){
            Ok(val) => Ok(val),
            Err(err) => Err(ApiError::new(Errors::RequestError, err.to_string())),
        }
    }
}

impl PlaylistTrait for Playlist {
    fn get_banner_url(&self) -> Option<String> {
        self.banner_url.clone()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_nextpage(&self) -> String {
        self.nextpage.clone()
    }

    fn get_thumbnail_url(&self) -> String {
        self.thumbnail_url.clone()
    }

    fn get_uploader_name(&self) -> String {
        self.uploader.clone()
    }

    fn get_uploader_avatar_url(&self) -> String {
        self.uploader_avatar.clone()
    }

    fn get_uploader_url(&self) -> String {
        self.uploader_url.clone()
    }

    fn get_video_count(&self) -> i32 {
        self.videos.clone()
    }

    fn get_videos(&self) -> Vec<Box<dyn crate::api::PartialVideoTrait>> {
        let mut ret_vec = Vec::new();
        for stream in self.related_streams.to_owned(){
            ret_vec.push(Box::new(stream) as Box<dyn PartialVideoTrait>);
        };
        ret_vec
    }

    fn get_id(&self) -> String {
        String::from("")
    }
}