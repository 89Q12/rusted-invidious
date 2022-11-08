use serde::Deserialize;
use serde_json::Value;
use serde_json;
use crate::api::{error::{ApiError, Errors}, SearchResultTrait};

use super::{SearchPlaylist, RelatedStream, channel::SearchChannel};

#[derive(Deserialize)]
pub struct Search {
    nextpage: String, // A JSON encoded page, which is used for the nextpage endpoint.
    suggestion: Option<String>, // Something something idk
    corrected: bool,  // Whether the query was corrected or not
    items: Vec<SearchItem>,
}
#[derive(Deserialize, Clone)]
pub enum SearchItem {
    SearchPlaylist(SearchPlaylist),
    SearchStream(RelatedStream),
    SearchChannel(SearchChannel)
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

impl SearchResultTrait for Search{
    fn get_items(&self) -> Vec<SearchItem> {
        self.items.to_owned()
    }

    fn get_suggestion(&self) -> Option<String> {
        self.suggestion.clone()
    }

    fn is_corrected(&self) -> bool {
        self.corrected.clone()
    }

    fn get_nextpage(&self) -> String {
        self.nextpage.clone()
    }
}