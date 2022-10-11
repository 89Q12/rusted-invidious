use crate::structs::{Playlist::Playlist, template_context::TemplateContext};
use askama::{Locale, Template};


#[derive(Template)] // this will generate the code...
#[template(path = "view_playlist.html")]
pub struct ViewPlaylist<'a>{
    #[locale]
    pub loc: Locale<'a>,
    pub context: TemplateContext<'a>,
    pub pl: Playlist
}