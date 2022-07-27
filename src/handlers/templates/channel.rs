use askama::{Template, Locale};

use super::base::Base;


#[derive(Template)] 
#[template(path = "channel.html",  print = "all")]
pub struct ChannelTemplate<'a>{
    #[locale]
    pub loc: Locale<'a>,
    pub _parent: &'a Base<'a>,
    pub channel: crate::structs::channel::Channel,
}