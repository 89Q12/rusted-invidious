use serde::Deserialize;
use serde_json::Value;

use super::{misc::RelatedStream, utils::parse_related};
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
impl Channel {
    pub(super) fn manual_parse(value: Value) -> Channel {
        Channel{
            avatar_url: value["avatar_url"].to_string(),
            banner_url: value["banner_url"].to_string(),
            description: value["description"].to_string(),
            id: value["avatar_url"].to_string(),
            name: value["avatar_url"].to_string(),
            nextpage: value["avatar_url"].to_string(),
            related_streams: parse_related(value),
            subscriber_count: value["subscriber_count"].to_string().parse().unwrap(),
            verified: value["verified"].as_bool().unwrap(),
        }
    }
}