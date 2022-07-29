use std::collections::{HashSet, HashMap};
use axum::{http::{StatusCode, Request},response::Response, body::{self, Body}};
use crate::{structs::template_context::TemplateContext, config::Config};
use regex::Regex;

pub fn render(html_str: Result<String, askama::Error>) -> Response { 
    return Response::builder()
    .status(StatusCode::OK)
    .body(body::boxed(match html_str{
        Ok(html) => body::Full::from(html),
        Err(err) => body::Full::from(err.to_string()),
    }))
    .unwrap();
}

pub fn result_to_body<T, E>(result: Result<T, E>) -> Response
where
    T: ToString,
    E: ToString,
{
    return Response::builder()
    .status(StatusCode::OK)
    .body(body::boxed(match result{
        Ok(t) => body::Full::from(t.to_string()),
        Err(err) => body::Full::from(err.to_string()),
    }))
    .unwrap();
}
pub fn string_to_body(str: String) -> Response
{
    return Response::builder()
    .status(StatusCode::OK)
    .body(body::boxed(body::Full::from(str)))
    .unwrap();
}
/// Function to find the differences between 2 vectors
/// https://stackoverflow.com/questions/63557089/is-there-a-built-in-function-to-compute-the-difference-of-two-sets
pub fn vect_difference(v1: &Vec<String>, v2: &Vec<&str>) -> Vec<String> {
    let s1: HashSet<String> = v1.iter().cloned().collect();
    let s2: HashSet<String> = v2.iter().cloned().map(|str| str.to_string()).collect();
    (&s1 - &s2).iter().cloned().collect()
}

/// Function to convert youtube url into our proxied urls
pub fn proxyfi_url(url: String, config: &Config) -> String {
    let proxy_domain = &config.proxy_domain;
    let re = Regex::new(r"(.*\.)?[a-zA-Z]+\.com").unwrap();
    re.replace(&url,proxy_domain).to_string()
}