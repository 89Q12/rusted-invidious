use super::{SearchPlaylist, RelatedStream};

pub struct Search {
    nextpage: String, // A JSON encoded page, which is used for the nextpage endpoint.
    suggestion: Option<String>, // Something something idk
    corrected: bool,  // Whether the query was corrected or not
    items: Vec<SearchItem>,
}

pub enum SearchItem {
    SearchPlaylist(SearchPlaylist),
    RelatedStream(RelatedStream)
}