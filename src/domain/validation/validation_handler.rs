use super::error::CustomError;

pub trait ValidationHandler {
    fn append(&mut self, an_error: &CustomError);
    fn append_validation_handler(&mut self, an_handler: &dyn ValidationHandler);
    fn has_errors(&self) -> bool {
        !self.get_errors().is_empty()
    }

    fn get_errors(&self) -> &[CustomError];

    fn get_first_error(&self) -> Option<CustomError> {
        if self.has_errors() {
            self.get_errors().get(0).map(|e| e.to_owned())
        } else {
            None
        }
    }
}

// pub trait Validation {
//     fn validate();
// }
