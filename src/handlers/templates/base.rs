use askama::{Template, Locale};
use crate::{config::{Config, Preferences}, structs::user::User};

askama::localization!(LOCALES);

#[derive(Template)] // this will generate the code...
#[template(path = "base.html")]
pub struct Base<'a>{
    title: String,
    config: &'a Config,
    user: Option<User>,
    preferences: Preferences,
    search_bar: Option<String>,
    current_page: String,
    #[locale]
    loc: Locale<'a>,
}
