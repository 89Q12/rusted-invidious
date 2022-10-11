use askama::{Template, Locale};
use crate::structs::template_context::TemplateContext;

#[derive(Template)] // this will generate the code...
#[template(path = "base.html")]
pub struct Base<'a>{
    #[locale]
    pub loc: Locale<'a>,
    pub context: TemplateContext<'a>
}
