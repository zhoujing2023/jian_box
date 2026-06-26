use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub struct MyError {
    message: String,
}

impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for MyError {}

impl MyError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}
