use super::Input;
use crate::domain::entity::Entity;
use crate::domain::identifier::Identifier;
use crate::domain::validation::error::CommonError;
use crate::domain::validation::validation_handler::ValidationHandler;
use crate::domain::validation::validator::Validator;

const NAME_MIN_LENGTH: usize = 1;
const NAME_MAX_LENGTH: usize = 255;

pub struct InputValidator<'a> {
    pub input: &'a Input<'a>,
    pub validation_handler: &'a mut dyn ValidationHandler,
}

impl<'a> InputValidator<'a> {
    pub fn new(input: &'a Input, validation_handler: &'a mut dyn ValidationHandler) -> Self {
        InputValidator {
            input,
            validation_handler,
        }
    }

    fn validate_id(&mut self) {
        let var = self.input.get_id().get_value().trim();

        if var.is_empty() {
            self.validation_handler.append(Box::new(CommonError {
                message: "'id' should not be empty".to_owned(),
            }));
        }
    }

    fn validate_name(&mut self) {
        let var = self.input.name.trim();

        if var.is_empty() {
            self.validation_handler.append(Box::new(CommonError {
                message: "'name' should not be empty".to_owned(),
            }));
        }

        if var.len() < NAME_MIN_LENGTH || var.len() > NAME_MAX_LENGTH {
            let message = format!(
                "'name' must be between {} and {} characters",
                NAME_MIN_LENGTH, NAME_MAX_LENGTH
            );
            self.validation_handler
                .append(Box::new(CommonError::new(&message)));
        }
    }

    fn validate_price(&mut self) {
        let var = self.input.price.to_owned();

        if var.is_empty() {
            self.validation_handler.append(Box::new(CommonError {
                message: "'price' should not be empty".to_owned(),
            }));
        }
    }
}

impl<'a> Validator for InputValidator<'a> {
    fn validate(&mut self) {
        self.validate_id();
        self.validate_name();
        self.validate_price();
    }
}
