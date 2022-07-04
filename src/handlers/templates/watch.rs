use askama::{Template, Locale};
use youtubei_rs::types::misc::StreamingData;

use crate::structs::Video::Video;

use super::base::Base;

#[derive(Template)] // this will generate the code...
#[template(path = "watch.html")]
pub struct Watch<'a>{
    title: String,
    #[locale]
    loc: Locale<'a>,
    _parent: Base<'a>,
    video: Video,
    streaming_data: StreamingData,
}
