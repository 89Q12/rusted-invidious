use serde::Deserialize;
use serde_json::Value;
use serde_json;
use crate::api::error::{ApiError, Errors};

use super::{SearchPlaylist, RelatedStream};

#[derive(Deserialize)]
pub struct Search {
    nextpage: String, // A JSON encoded page, which is used for the nextpage endpoint.
    suggestion: Option<String>, // Something something idk
    corrected: bool,  // Whether the query was corrected or not
    items: Vec<SearchItem>,
}
#[derive(Deserialize)]
pub enum SearchItem {
    SearchPlaylist(SearchPlaylist),
    RelatedStream(RelatedStream)
}

impl TryFrom<Value> for Search{
    type Error = ApiError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match serde_json::from_value(value){
            Ok(val) => Ok(val),
            Err(err) => Err(ApiError::new(Errors::RequestError, err.to_string())),
        }
    }
}
