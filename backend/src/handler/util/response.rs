use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct ErrResp<'a> {
    message: &'a str,
}

impl<'a> ErrResp<'a> {
    pub fn new(message: &'a str) -> Self {
        Self { message }
    }
}