use serde::{Serialize};
#[derive(Serialize)]
pub struct Response {
    pub message: String
}

impl Response {
    pub fn new<S: Into<String>>(message: S) -> Response {
        Response {
            message: message.into()
        }
    }
}
