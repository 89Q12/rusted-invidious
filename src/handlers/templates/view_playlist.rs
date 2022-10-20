use askama::{Locale, Template};
use crate::api::PlaylistTrait;

use super::TemplateContext;

#[derive(Template)] // this will generate the code...
#[template(path = "view_playlist.html")]
pub struct ViewPlaylist<'a, PL: PlaylistTrait>{
    #[locale]
    pub loc: Locale<'a>,
    pub context: TemplateContext<'a>,
    pub pl: PL,
    pub playlist_id: String
}