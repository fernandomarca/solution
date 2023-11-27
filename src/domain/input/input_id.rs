use crate::domain::identifier::Identifier;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct InputId {
    value: String,
}

impl Identifier for InputId {
    fn get_value(&self) -> &str {
        self.value.as_ref()
    }
}

impl InputId {
    fn new(value: &str) -> Self {
        InputId {
            value: value.to_string(),
        }
    }
    pub fn unique() -> Self {
        let value = Uuid::new_v4().to_string();
        InputId::new(&value)
    }

    pub fn from_str(an_id: &str) -> Self {
        InputId::new(an_id)
    }

    pub fn from_uuid(an_id: Uuid) -> Self {
        InputId::new(an_id.to_string().as_str())
    }
}
