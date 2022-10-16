pub enum Errors {
    Parse,
    Request,
}
pub enum Actions {
    TryInvidious,
    TryPiped,
    RenderError,
}

pub struct ApiError{
    kind: Errors,
    recoverable: bool,
    message: String,
    action: Actions
}
impl ApiError {
    pub fn new(kind: Errors, recoverable: bool, message: String, action: Actions) -> Self { Self { kind, recoverable, message, action } }
    pub fn kind(&self) -> Errors{
        self.kind
    }
    pub fn recoverable(&self) -> bool{
        self.recoverable
    }
    pub fn message(&self) -> String{
        self.message
    }
    pub fn action(&self) -> Actions{
        self.action
    }
}