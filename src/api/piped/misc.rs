use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelatedStream{
    duration: i32, // The duration of the related video in seconds
    thumbnail:String, // The thumbnail of the related video
    title: String, // The title of the related video
    uploaded_date: String, // The date the related video was uploaded
    uploader_avatar: String, // The avatar of the channel of the related video
    uploader_url: String, // The URL of the channel of the related video
    uploader_verified: bool, // Whether or not the channel of the related video is verified
    url: String, // The URL of the related video
    views: i32, // The number of views the related video has
    short_description: Option<String>,
    uploader_name: String, // Author name aka Channel name
    uploaded: i64, // Unix timestamp. I know 32bit but I dont wanna need to change it in 3038 lol
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Next {
    nextpage: String,
    related_streams: Vec<RelatedStream>,
}
impl From<Value> for Next{
    fn from(value: Value) -> Self {
        serde_json::from_value(value).unwrap()
    }
}