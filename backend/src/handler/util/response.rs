use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct ErrorMessage<'a> {
    message: &'a str,
}

impl<'a> ErrorMessage<'a> {
    pub fn new(message: &'a str) -> Self {
        Self { message }
    }
}