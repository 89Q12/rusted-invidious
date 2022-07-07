use chrono::Utc;

pub struct TemplateContext{
    pub continue_autoplay: bool,
    pub autoplay: bool,
    pub query_params: Vec<String>,
    pub listen: bool,
    pub current_page: String,
    pub nojs: bool,
}