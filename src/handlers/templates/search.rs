use askama::{Template, Locale};
use youtubei_rs::types::enums::ItemSectionRendererContents;

use crate::structs::template_context::TemplateContext;


#[derive(Template)] 
#[template(path = "search.html",  print = "all")]
pub struct Search<'a>{
    #[locale]
    pub loc: Locale<'a>,
    pub context: TemplateContext<'a>,
    pub videos: Vec<ItemSectionRendererContents>
}