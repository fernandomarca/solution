use super::identifier::Identifier;
use super::validation::validation_handler::ValidationHandler;

pub trait Entity {
    type Id: Identifier;
    fn get_id(&self) -> &Self::Id;

    fn validate(&self, handler: &mut dyn ValidationHandler);
}
