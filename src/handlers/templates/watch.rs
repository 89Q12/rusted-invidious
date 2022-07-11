use askama::{Locale, Template};

use crate::structs::{Playlist::Playlist, Video::Video, player::Player};

use super::base::Base;

#[derive(Template)] // this will generate the code...
#[template(path = "watch.html")]
pub struct Watch<'a> {
    #[locale]
    pub loc: Locale<'a>,
    pub _parent: Base<'a>,
    pub video: Video,
    pub player: Player,
    pub playlist: Option<Playlist>,
    pub comment_html: String,
}