use askama::{Locale, Template};
use youtubei_rs::types::playlist::NextPlaylist;

use crate::structs::{Video::Video, player::Player};

use super::base::Base;

#[derive(Template)] // this will generate the code...
#[template(path = "watch.html")]
pub struct Watch<'a> {
    #[locale]
    pub loc: Locale<'a>,
    pub _parent: Base<'a>,
    pub video: Video<'a>,
    pub player: Player<'a>,
    pub playlist: Option<NextPlaylist>,
    pub comment_html: String,
}