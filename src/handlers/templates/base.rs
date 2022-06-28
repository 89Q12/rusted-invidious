use askama::{Template, Locale};
use crate::{config::{Config, Preferences}, structs::user::User};

askama::localization!(LOCALES);

#[derive(Template)] // this will generate the code...
#[template(path = "base.html")]
pub struct Base<'a>{
    title: String,
    config: Config,
    user: Option<User>,
    preferences: Preferences,
    headers: Vec<String>,
    search_bar: Option<String>,
    current_page: String,
    #[locale]
    loc: Locale<'a>,
}
