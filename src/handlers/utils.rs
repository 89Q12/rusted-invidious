use std::collections::HashSet;

use axum::{http::{StatusCode, Request},response::Response, body::{self, Body}};

use crate::structs::template_context::TemplateContext;

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
pub fn build_params(request: &Request<Body>) -> TemplateContext{
    match request.uri().query(){
        Some(val) => {
            TemplateContext{
                continue_autoplay: val.contains("continue=1"),
                autoplay: request.uri().query().unwrap().contains("autoplay=1"),
                listen: val.contains("listen=1"),
                query_params: val.split("&").map(|str| str.to_string()).collect::<Vec<String>>(),
                current_page: request.uri().path().to_string(),
                nojs: val.contains("nojs=1"),
            }
        },
        None =>  TemplateContext{
            continue_autoplay: false,
            autoplay: false,
            query_params: Vec::new(),
            listen: false,
            current_page:  request.uri().path().to_string(),
            nojs: false,
        },
    }

}
/// Function to find the differences between 2 vectors
/// https://stackoverflow.com/questions/63557089/is-there-a-built-in-function-to-compute-the-difference-of-two-sets
pub fn vect_difference(v1: &Vec<String>, v2: &Vec<&str>) -> Vec<String> {
    let s1: HashSet<String> = v1.iter().cloned().collect();
    let s2: HashSet<String> = v2.iter().cloned().map(|str| str.to_string()).collect();
    (&s1 - &s2).iter().cloned().collect()
}