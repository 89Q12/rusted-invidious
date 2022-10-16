use askama::{Template, Locale};

use crate::api::piped::SearchItem;
use super::TemplateContext;


#[derive(Template)] 
#[template(path = "search.html",  print = "all")]
pub struct Search<'a>{
    #[locale]
    pub loc: Locale<'a>,
    pub context: TemplateContext<'a>,
    pub items: Vec<SearchItem> 
}