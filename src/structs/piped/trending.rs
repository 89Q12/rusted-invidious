use serde::Deserialize;
use serde_json::Value;

use super::related_streams::RelatedStream;

#[derive(Deserialize)]
pub struct Trending{
    videos: Vec<RelatedStream>,
}

impl From<Value> for Trending{
    fn from(value: Value) -> Self {
        let videos: Vec<RelatedStream> = serde_json::from_value(value).unwrap();
        Self{
            videos
        }
    }
}