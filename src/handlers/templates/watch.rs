use askama::{Locale, Template};
use crate::api::piped::{Video, Playlist};

use super::TemplateContext;

#[derive(Template)] // this will generate the code...
#[template(path = "watch.html")]
pub struct Watch<'a> {
    #[locale]
    pub loc: Locale<'a>,
    pub context: TemplateContext<'a>,
    pub video: Video,
    pub playlist: Option<Playlist>
}