use askama::{Template, Locale};
use crate::{config::{Config, Preferences}, structs::{user::User, template_context::TemplateContext}};

#[derive(Template)] // this will generate the code...
#[template(path = "base.html")]
pub struct Base<'a>{
    pub title: String,
    pub config: &'a Config,
    pub user: Option<User>,
    pub preferences: &'a Preferences,
    #[locale]
    pub loc: Locale<'a>,
    pub params: TemplateContext
}
