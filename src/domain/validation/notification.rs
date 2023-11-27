#![allow(dead_code)]
use super::error::CommonError;
use super::validation_handler::ValidationHandler;
use std::error::Error;

#[derive(Debug, Default)]
pub struct Notification {
    pub errors: Vec<Box<dyn Error>>,
}

impl Notification {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn with_errors(errors: Vec<Box<dyn Error>>) -> Self {
        Self { errors }
    }

    pub fn with_one_error(error: Box<dyn Error>) -> Self {
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
    fn append(&mut self, error: Box<dyn Error>) {
        let error_result = error;
        self.errors
            .push(Box::new(CommonError::new(&error_result.to_string())));
    }

    fn append_validation_handler(&mut self, an_handler: &dyn ValidationHandler) {
        for error in an_handler.get_errors() {
            self.append(Box::new(CommonError::new(&error.to_string())));
        }
    }

    fn get_errors(&self) -> &Vec<Box<dyn Error>> {
        self.errors.as_ref()
    }
}
