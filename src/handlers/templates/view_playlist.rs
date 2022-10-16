use askama::{Locale, Template};
use crate::api::piped::Playlist;

use super::TemplateContext;

#[derive(Template)] // this will generate the code...
#[template(path = "view_playlist.html")]
pub struct ViewPlaylist<'a>{
    #[locale]
    pub loc: Locale<'a>,
    pub context: TemplateContext<'a>,
    pub pl: Playlist
}