use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelatedStream{
    pub duration: i32, // The duration of the related video in seconds
    pub thumbnail:String, // The thumbnail of the related video
    pub title: String, // The title of the related video
    pub uploaded_date: String, // The date the related video was uploaded
    pub uploader_avatar: String, // The avatar of the channel of the related video
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
impl From<Value> for Next{
    fn from(value: Value) -> Self {
        serde_json::from_value(value).unwrap()
    }
}

impl RelatedStream{
    pub fn get_id(&self) -> String{
        self.url.split("=").map(|x| x.to_string()).collect::<Vec<String>>()[1]
    }
    pub fn live_now(&self) -> bool{
        return self.duration == -1 && self.uploaded == -1
    }
}