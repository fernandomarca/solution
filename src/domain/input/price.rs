use crate::domain::value_object::ValueObject;
use num_format::Locale;
use num_format::ToFormattedString;

/*
Price{
  unit:"kg"
  value:u64,
}
*/

#[derive(Debug, Clone)]
pub struct Price {
    unit: String,
    value: u64,
}

impl ValueObject for Price {}

impl Price {
    pub fn new(unit: &str, value: u64) -> Self {
        Price {
            unit: unit.to_string(),
            value,
        }
    }
    pub fn get_value_formatted(&self) -> String {
        let value = self.value;

        let reais = value / 100;
        let centavos = value % 100;

        let formatted = format!("{},{:02}", reais.to_formatted_string(&Locale::pt), centavos);
        formatted
    }
    pub fn get_value(&self) -> u64 {
        self.value
    }
    pub fn get_unit(&self) -> &str {
        &self.unit
    }
}

impl std::fmt::Display for Price {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unit:{} value:{}", self.get_unit(), self.get_value())
    }
}
