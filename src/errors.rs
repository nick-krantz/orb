use std::{error, fmt, result};

/// Custom error message with both a user friendly message and technical details
#[derive(Debug)]
pub struct ErrorWithContext {
    pub user_message: String,
    pub technical_details: Option<String>,
}

impl ErrorWithContext {
    pub fn new(
        user_message: impl Into<String>,
        technical_details: Option<impl Into<String>>,
    ) -> Self {
        Self {
            user_message: user_message.into(),
            technical_details: technical_details.map(|d| d.into()),
        }
    }
}

impl fmt::Display for ErrorWithContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.user_message)
    }
}

impl error::Error for ErrorWithContext {}

pub type Result<T> = result::Result<T, ErrorWithContext>;
