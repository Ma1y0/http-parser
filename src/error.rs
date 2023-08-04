use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct ParsingError {
    msg: String,
}

impl Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for ParsingError {
    fn description(&self) -> &str {
        &self.msg
    }
}

impl ParsingError {
    pub fn new(msg: &str) -> Self {
        Self {
            msg: msg.to_string(),
        }
    }
}
