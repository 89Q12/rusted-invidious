use askama::{Locale, Template};
use crate::api::{PlaylistTrait, VideoBasicInfoTrait};

use super::TemplateContext;

#[derive(Template)] // this will generate the code...
#[template(path = "watch.html")]
pub struct Watch<'a> {
    #[locale]
    pub loc: Locale<'a>,
    pub context: TemplateContext<'a>,
    pub video: Box<dyn VideoBasicInfoTrait>,
    pub playlist: Option<Box<dyn PlaylistTrait>>,
    pub video_id: String,
}