use serde::Deserialize;
use serde_json::Value;

use crate::api::ChannelTrait;
use crate::api::error::{ApiError, Errors};
use crate::api::PartialVideoTrait;
use super::misc::RelatedStream;
//Represent the video tab ONLY
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Channel{
    pub avatar_url: String, // The avatar of the channel
    pub banner_url: String, // The banner of the channel
    pub description: String, // The description of the channel
    pub id: String, // The ID of the channel
    pub name: String, // The name of the channel
    pub nextpage: String, // A JSON encoded page, which is used for the nextpage endpoint.
    pub related_streams: Vec<RelatedStream>, // A list of videos from the channel
    pub subscriber_count: i32, // The number of subscribers the channel has
    pub verified: bool // Whether or not the channel is verified
}

impl TryFrom<Value> for Channel{
    type Error = ApiError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match serde_json::from_value(value){
            Ok(val) => Ok(val),
            Err(err) => Err(ApiError::new(Errors::RequestError, err.to_string())),
        }
    }
}
impl ChannelTrait for Channel {
    fn get_avatar_url(&self) -> String {
        self.avatar_url.clone()
    }

    fn get_banner_url(&self) -> String {
        self.banner_url.clone()
    }

    fn get_description(&self) -> String {
        self.description.clone()
    }

    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_nextpage(&self) -> String {
        self.nextpage.clone()
    }

    fn get_related_streams(&self) -> Vec<Box<dyn PartialVideoTrait>> {
        let mut ret_vec = Vec::new();
        for stream in self.related_streams.to_owned(){
            ret_vec.push(Box::new(stream) as Box<dyn PartialVideoTrait>);
        };
        ret_vec
    }

    fn get_subscriber_count(&self) -> i32 {
        self.subscriber_count.clone()
    }

    fn is_verified(&self) -> bool {
        self.verified.clone()
    }

}