use serde::Deserialize;
use serde_json::Value;

use crate::api::{error::{ApiError, Errors}, PartialVideoTrait};

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RelatedStream{
    pub duration: i32, // The duration of the related video in seconds
    pub thumbnail:String, // The thumbnail of the related video
    pub title: String, // The title of the related video
    pub uploaded_date: String, // The date the related video was uploaded
    pub uploader_avatar: Option<String>, // The avatar of the channel of the related video
    pub uploader_url: String, // The URL of the channel of the related video
    pub uploader_verified: bool, // Whether or not the channel of the related video is verified
    pub url: String, // The URL of the related video
    pub views: i32, // The number of views the related video has
    pub short_description: Option<String>,
    pub uploader_name: String, // Author name aka Channel name
    pub uploaded: i64, // Unix timestamp. I know 32bit but I dont wanna need to change it in 3038 lol
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Next {
    nextpage: String,
    related_streams: Vec<RelatedStream>,
}
impl TryFrom<Value> for Next{
    type Error = ApiError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match serde_json::from_value(value){
            Ok(val) => Ok(val),
            Err(err) => Err(ApiError::new(Errors::RequestError, err.to_string())),
        }
    }
}

impl PartialVideoTrait for RelatedStream{
    fn get_id(&self) -> String {
        self.url.split("=").map(|x| x.to_string()).collect::<Vec<String>>()[1].clone()
    }

    fn get_short_description(&self) -> String {
        match &self.short_description{
            Some(val) => val.clone(),
            None => String::from(""),
        }
    }
    fn is_live(&self) -> bool {
        return self.duration == -1 && self.uploaded == -1
    }

    fn get_title(&self) -> String {
        self.title.clone()
    }

    fn get_upload_date(&self) -> String {
        self.uploaded_date.clone()
    }

    fn get_uploader_name(&self) -> String {
        self.uploader_name.clone()
    }

    fn is_uploader_verified(&self) -> bool {
        self.uploader_verified.clone()
    }

    fn get_uploader_avatar_url(&self) -> String {
        match &self.uploader_avatar{
            Some(val) => val.clone(),
            None => String::from(""),
        }
    }

    fn get_url(&self) -> String {
        self.url.clone()
    }

    fn get_thumbnail_url(&self) -> String {
        self.thumbnail.clone()
    }

    fn get_duration(&self) -> i32 {
        self.duration.clone()
    }

    fn get_uploader_url(&self) -> String {
        self.uploader_url.clone()
    }

    fn get_views(&self) -> i32 {
        self.views.clone()
    }
}