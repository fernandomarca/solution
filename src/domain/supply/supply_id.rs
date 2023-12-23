use crate::domain::identifier::Identifier;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct SupplyId {
    value: String,
}

impl Identifier for SupplyId {
    fn get_value(&self) -> &str {
        self.value.as_ref()
    }
}

impl SupplyId {
    fn new(value: &str) -> Self {
        SupplyId {
            value: value.to_string(),
        }
    }
    pub fn unique() -> Self {
        let value = Uuid::new_v4().to_string();
        SupplyId::new(&value)
    }

    pub fn from_str(an_id: &str) -> Self {
        SupplyId::new(an_id)
    }

    pub fn from_uuid(an_id: Uuid) -> Self {
        SupplyId::new(an_id.to_string().as_str())
    }
}
