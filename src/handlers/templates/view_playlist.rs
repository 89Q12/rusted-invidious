use crate::structs::Playlist::Playlist;
use askama::{Locale, Template};
use super::base::Base;

#[derive(Template)] // this will generate the code...
#[template(path = "view_playlist.html")]
pub struct ViewPlaylist<'a>{
    #[locale]
    pub loc: Locale<'a>,
    pub _parent: Base<'a>,
    pub pl: Playlist
}