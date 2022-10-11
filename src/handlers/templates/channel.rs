use askama::{Template, Locale};
use crate::structs::template_context::TemplateContext;

#[derive(Template)] 
#[template(path = "channel.html",  print = "all")]
pub struct ChannelTemplate<'a>{
    #[locale]
    pub loc: Locale<'a>,
    pub channel: crate::structs::channel::Channel,
    pub context: TemplateContext<'a>
}