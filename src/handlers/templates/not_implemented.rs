use askama::{Locale, Template};
use super::TemplateContext;
#[derive(Template)]
#[template(path = "not_implemented.html")]
pub struct NotImplemented<'a> {
    #[locale]
    pub loc: Locale<'a>,
    pub context: TemplateContext<'a>,
}