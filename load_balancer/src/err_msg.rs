use serde::Serialize;

#[derive(Serialize)]
pub struct ErrMsg {
    message: String,
}

impl ErrMsg {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}