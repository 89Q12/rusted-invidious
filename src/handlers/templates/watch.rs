use askama::{Locale, Template};
use youtubei_rs::types::playlist::NextPlaylist;

use crate::structs::{video::Video, player::Player, template_context::TemplateContext};

#[derive(Template)] // this will generate the code...
#[template(path = "watch.html")]
pub struct Watch<'a> {
    #[locale]
    pub loc: Locale<'a>,
    pub context: TemplateContext<'a>,
    pub video: Video,
}