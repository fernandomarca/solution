#![allow(dead_code)]
use super::error::CustomError;
use super::validation_handler::ValidationHandler;

#[derive(Debug, Default)]
pub struct Notification {
    pub errors: Vec<CustomError>,
}

impl Notification {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn with_errors(errors: Vec<CustomError>) -> Self {
        Self { errors }
    }

    pub fn with_one_error(error: CustomError) -> Self {
        Self {
            errors: vec![error],
        }
    }

    pub fn format_errors(&self) -> Vec<String> {
        self.get_errors()
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
    }
}

impl ValidationHandler for Notification {
    fn append(&mut self, error: &CustomError) {
        self.errors.push(error.to_owned());
    }

    fn append_validation_handler(&mut self, an_handler: &dyn ValidationHandler) {
        for error in an_handler.get_errors() {
            self.append(error);
        }
    }

    fn get_errors(&self) -> &[CustomError] {
        self.errors.as_ref()
    }
}
