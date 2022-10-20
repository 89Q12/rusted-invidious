use askama::{Locale, Template};
use crate::api::error::ApiError;

use super::TemplateContext;
#[derive(Template)]
#[template(path = "error.html")]
pub struct Error<'a> {
    #[locale]
    pub loc: Locale<'a>,
    pub context: TemplateContext<'a>,
    pub error: ApiError
}