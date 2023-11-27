use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct CommonError {
    pub message: String,
}

impl CommonError {
    pub fn new(message: &str) -> CommonError {
        CommonError {
            message: message.to_string(),
        }
    }
}

impl std::fmt::Display for CommonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CommonError {}
