use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Errors {
    ParsingError,
    RequestError,
}
#[derive(Debug)]
pub struct ApiError{
    kind: Errors,
    message: String,
}
impl ApiError {
    pub fn new(kind: Errors, message: String) -> Self { Self { kind, message } }
    pub fn kind(&self) -> Errors{
        self.kind.clone()
    }
    pub fn message(&self) -> String{
        self.message.clone()
    }
}

impl Display for Errors{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Errors::ParsingError => write!(f, "Error occured during parsing."),
            Errors::RequestError => write!(f, "Error occured during the request."),
        }
    }
}