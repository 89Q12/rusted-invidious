use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct TemplateContext{
    pub continue_autoplay: bool,
    pub autoplay: bool,
    pub query_params: Vec<String>,
    pub listen: bool,
    pub current_page: String,
    pub nojs: bool,
    pub local: bool,
    pub controls: bool,
    pub search_query: Option<String>,
}