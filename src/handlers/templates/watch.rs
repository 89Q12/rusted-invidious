use askama::{Locale, Template};
use youtubei_rs::types::misc::StreamingData;

use crate::structs::{Playlist::Playlist, Video::Video};

use super::base::Base;

#[derive(Template)] // this will generate the code...
#[template(path = "watch.html")]
pub struct Watch<'a> {
    #[locale]
    pub loc: Locale<'a>,
    pub _parent: Base<'a>,
    pub video: Video,
    pub streaming_data: StreamingData,
    pub playlist: Option<Playlist>,
    pub comment_html: String,
}

