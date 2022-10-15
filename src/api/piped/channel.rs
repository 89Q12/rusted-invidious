use serde::Deserialize;
use serde_json::Value;

use super::related_streams::RelatedStream;
//Represent the video tab ONLY
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Channel{
    avatar_url: String, // The avatar of the channel
    banner_url: String, // The banner of the channel
    description: String, // The description of the channel
    id: String, // The ID of the channel
    name: String, // The name of the channel
    nextpage: String, // A JSON encoded page, which is used for the nextpage endpoint.
    related_streams: Vec<RelatedStream>, // A list of videos from the channel
    subscriber_count: i32, // The number of subscribers the channel has
    verified: bool // Whether or not the channel is verified
}

impl From<Value> for Channel{
    fn from(value: Value) -> Self {
        serde_json::from_value(value).unwrap()
    }
}