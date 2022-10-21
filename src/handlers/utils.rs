use axum::{http::StatusCode,response::Response, body};
use askama::{langid, Template};
use crate::api::error::ApiError;
use crate::handlers::templates::TemplateContext;
askama::localization!(LOCALES);
pub fn render(html_str: Result<String, askama::Error>) -> Response { 
    return Response::builder()
    .status(StatusCode::OK)
    .body(body::boxed(match html_str{
        Ok(html) => body::Full::from(html),
        Err(err) => body::Full::from(err.to_string()),
    }))
    .unwrap();
}

pub fn render_error(error: ApiError,context: TemplateContext)-> Response{
    render(super::templates::error::Error{
        loc: askama::Locale::new(langid!("en-US"), &LOCALES),
        context,
        error,
    }.render())
}

pub fn render_not_implemented(context: TemplateContext) -> Response {
    render(super::templates::not_implemented::NotImplemented{
        loc: askama::Locale::new(langid!("en-US"), &LOCALES),
        context,
    }.render())
}