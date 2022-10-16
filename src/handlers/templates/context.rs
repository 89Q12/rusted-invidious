use std::collections::HashMap;

use crate::{
    config::{Config, Preferences},
    api::user::User,
};
use axum::{body::Body, http::request::Request};
/// Will be used as the context for the every template that extends base.html
pub struct Context<'a> {
    pub query_params: HashMap<String, String>,
    pub config: &'a Config,
    pub user: Option<User>,
    pub preferences: Preferences,
    pub current_page: String,
}

impl<'a> Context<'a> {
    pub fn new(
        request: &'a Request<Body>,
        user: Option<User>,
        config: &'a Config,
    ) -> Context<'a> {
        let mut params: Vec<String> = Vec::new();
        let query_params = match request.uri().query() {
            Some(val) => {
                let mut params_map: HashMap<String, String> = HashMap::new();
                params = val
                    .split("&")
                    .map(|str| str.to_string())
                    .collect::<Vec<String>>();
                params.iter().for_each(|p| {
                    let split_param: Vec<String> =
                        p.split("=").map(|str| str.to_string()).collect();
                    let key = match split_param.get(0) {
                        Some(key) => key,
                        None => "",
                    };
                    let value = match split_param.get(1) {
                        Some(value) => value,
                        None => "",
                    };
                    if !key.is_empty() && !value.is_empty() {
                        params_map.insert(key.to_string(), value.to_string());
                    }
                });
                params_map
            }
            None => HashMap::new(),
        };
        let preferences = match user.as_ref() {
            Some(user) => user.preferences.clone(),
            None => config.default_preferences.clone(),
        };
        Context {
            query_params,
            config,
            user,
            preferences,
            current_page: request.uri().path().to_string() + &params.join("&"),
        }
    }

    pub fn continue_autoplay(&self) -> bool {
        match self.query_params.get("continue") {
            Some(value) =>  match value.as_str(){
                "1" => true,
                "0" => false,
                _ => false
            },
            None => false,
        }
    }
    pub fn autoplay(&self) -> bool {
        match self.query_params.get("autoplay") {
            Some(value) =>  match value.as_str(){
                "1" => true,
                "0" => false,
                _ => false
            },
            None => false,
        }
    }
    pub fn listen(&self) -> bool {
        match self.query_params.get("listen") {
            Some(value) =>  match value.as_str(){
                "1" => true,
                "0" => false,
                _ => false
            },
            None => false,
        }
    }
    pub fn no_js(&self) -> bool {
        match self.query_params.get("nojs") {
            Some(value) =>  match value.as_str(){
                "1" => true,
                "0" => false,
                _ => false
            },
            None => false,
        }
    }
    pub fn proxy_local(&self) -> bool {
        match self.query_params.get("local") {
            Some(value) =>  match value.as_str(){
                "1" => true,
                "0" => false,
                _ => false
            },
            None => false,
        }
    }
    pub fn show_controls(&self) -> bool {
        match self.query_params.get("controls") {
            Some(value) =>  match value.as_str(){
                "1" => true,
                "0" => false,
                _ => false
            },
            None => false,
        }
    }
    pub fn search_query(&self) -> String {
        match self.query_params.get("q") {
            Some(value) => value.clone(),
            None => String::from(""),
        }
    }
    pub fn query_params(&self) -> String {
        self.query_params.to_owned().into_iter().map(|(k,v)| k+ "="+&v).collect::<Vec<String>>().join("&")
    }
}
