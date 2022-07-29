use askama::{Template, Locale};
use youtubei_rs::types::enums::ItemSectionRendererContents;

use super::base::Base;

#[derive(Template)] 
#[template(path = "feeds/trending.html")]
pub struct Trending<'a>{
    #[locale]
    pub loc: Locale<'a>,
    pub _parent: &'a Base<'a>,
    pub trending_items: Vec<ItemSectionRendererContents>

}