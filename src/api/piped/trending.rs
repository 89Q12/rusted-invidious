use serde::Deserialize;
use serde_json::Value;

use crate::api::{error::{ApiError, Errors}, TrendingTrait,PartialVideoTrait};

use super::misc::RelatedStream;

#[derive(Deserialize)]
pub struct Trending{
    videos: Vec<RelatedStream>,
}

impl TryFrom<Value> for Trending{
    type Error = ApiError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match serde_json::from_value(value){
            Ok(val) => Ok(val),
            Err(err) => Err(ApiError::new(Errors::RequestError, err.to_string())),
        }
    }
}
impl TrendingTrait for Trending{
    fn get_videos(self) -> Vec<Box<dyn crate::api::PartialVideoTrait>> {
        let mut ret_vec = Vec::new();
        for stream in self.videos{
            ret_vec.push(Box::new(stream) as Box<dyn PartialVideoTrait>);
        };
        ret_vec
    }
}