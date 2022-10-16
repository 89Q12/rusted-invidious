use askama::{Template, Locale};
use super::TemplateContext;

#[derive(Template)] 
#[template(path = "channel.html",  print = "all")]
pub struct ChannelTemplate<'a>{
    #[locale]
    pub loc: Locale<'a>,
    pub channel: crate::api::piped::Channel,
    pub context: TemplateContext<'a>
}