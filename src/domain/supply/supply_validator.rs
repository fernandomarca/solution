use super::Supply;
use crate::domain::entity::Entity;
use crate::domain::identifier::Identifier;
use crate::domain::validation::error::CustomError;
use crate::domain::validation::validation_handler::ValidationHandler;
use crate::domain::validation::validator::Validator;

const NAME_MIN_LENGTH: usize = 1;
const NAME_MAX_LENGTH: usize = 255;

pub struct SupplyValidator<'a> {
    pub supply: &'a Supply<'a>,
    pub validation_handler: &'a mut dyn ValidationHandler,
}

impl<'a> SupplyValidator<'a> {
    pub fn new(supply: &'a Supply, validation_handler: &'a mut dyn ValidationHandler) -> Self {
        SupplyValidator {
            supply,
            validation_handler,
        }
    }

    fn validate_id(&mut self) {
        let var = self.supply.get_id().get_value().trim();

        if var.is_empty() {
            self.validation_handler
                .append(&CustomError::Error("'id' should not be empty".to_string()));
        }
    }

    fn validate_name(&mut self) {
        let var = self.supply.name.trim();

        if var.is_empty() {
            self.validation_handler.append(&CustomError::Error(
                "'name' should not be empty".to_string(),
            ));
        }

        if var.len() < NAME_MIN_LENGTH || var.len() > NAME_MAX_LENGTH {
            let message = format!(
                "'name' must be between {} and {} characters",
                NAME_MIN_LENGTH, NAME_MAX_LENGTH
            );
            self.validation_handler.append(&CustomError::Error(message));
        }
    }

    fn validate_price(&mut self) {
        let var = self.supply.price.to_owned();

        if var.is_empty() {
            self.validation_handler.append(&CustomError::Error(
                "'price' should not be empty".to_string(),
            ));
        }
    }
}

impl<'a> Validator for SupplyValidator<'a> {
    fn validate(&mut self) {
        self.validate_id();
        self.validate_name();
        self.validate_price();
    }
}
