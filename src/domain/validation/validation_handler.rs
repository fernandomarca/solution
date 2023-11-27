use std::error::Error;

pub trait ValidationHandler {
    fn append(&mut self, an_error: Box<dyn Error>);
    fn append_validation_handler(&mut self, an_handler: &dyn ValidationHandler);
    fn has_errors(&self) -> bool {
        !self.get_errors().is_empty()
    }

    fn get_errors(&self) -> &Vec<Box<dyn Error>>;

    fn get_first_error(&self) -> Option<&dyn Error> {
        if self.has_errors() {
            self.get_errors().get(0).map(|e| e.as_ref())
        } else {
            None
        }
    }
}

// pub trait Validation {
//     fn validate();
// }
