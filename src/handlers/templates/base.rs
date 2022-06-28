use askama::Template;
use crate::{database::models::user::User, config::{Config, Preferences}};


#[derive(Template)] // this will generate the code...
#[template(path = "base.html")]
pub struct Base{
    title: String,
    config: Config,
    user: User,
    preferences: Preferences,
}
