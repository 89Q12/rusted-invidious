use askama::{Template, Locale};
use super::TemplateContext;
use crate::api::ChannelTrait;
#[derive(Template)] 
#[template(path = "channel.html",  print = "all")]
pub struct ChannelTemplate<'a>{
    #[locale]
    pub loc: Locale<'a>,
    pub channel: Box<dyn ChannelTrait>,
    pub context: TemplateContext<'a>
}