use askama::{Template, Locale};
use youtubei_rs::types::enums::ItemSectionRendererContents;

use super::base::Base;


#[derive(Template)] 
#[template(path = "search.html",  print = "all")]
pub struct Search<'a>{
    #[locale]
    pub loc: Locale<'a>,
    pub _parent: &'a Base<'a>,
    pub videos: Vec<ItemSectionRendererContents>
}