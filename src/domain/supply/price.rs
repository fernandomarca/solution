use crate::domain::value_object::ValueObject;
use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;

/*
Price{
  unit:"kg"
  value:u64,
}
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Price {
    unit: String,

    #[serde(with = "rust_decimal::serde::float")]
    value: Decimal,
}

impl ValueObject for Price {}

impl Price {
    pub fn new(unit: &str, value: i64) -> Self {
        Price {
            unit: unit.to_string(),
            value: Decimal::new(value, 2),
        }
    }
    pub fn from_dec(unit: &str, value: Decimal) -> Self {
        let mut value = value.to_owned();
        value.set_scale(2).unwrap();
        Price {
            unit: unit.to_string(),
            value,
        }
    }

    pub fn from_str(unit: &str, value: &str) -> Self {
        let mut value = Decimal::from_str_exact(value).unwrap();
        value.set_scale(2).unwrap();
        Price {
            unit: unit.to_string(),
            value,
        }
    }
    pub fn get_value_formatted(&self) -> String {
        self.value.to_string()
    }
    pub fn get_value(&self) -> Decimal {
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

#[cfg(test)]
mod price_tests {
    use rust_decimal::prelude::FromPrimitive;
    use rust_decimal::Decimal;
    use rust_decimal_macros::dec;
    use serde_json::json;

    #[test]
    fn create_price_one_cent_test() {
        use super::Price;
        let price = Price::new("kg", 1);
        assert_eq!(price.get_value_formatted(), "0.01");
        assert_eq!(price.get_value(), dec!(0.01));
        assert_eq!(price.get_unit(), "kg");
    }

    #[test]
    fn create_price_from_f64_one_cent_test() {
        use super::Price;
        let value = 1.0;
        let price = Price::from_dec("kg", Decimal::from_f64(value).unwrap());
        assert_eq!(price.get_value_formatted(), "0.01");
        assert_eq!(price.get_value(), dec!(0.01));
        assert_eq!(price.get_unit(), "kg");
    }

    #[test]
    fn create_price_from_f64_str_one_cent_test() {
        use super::Price;
        let value = 1.0.to_string();
        let price = Price::from_str("kg", &value);
        assert_eq!(price.get_value_formatted(), "0.01");
        assert_eq!(price.get_value(), dec!(0.01));
        assert_eq!(price.get_unit(), "kg");
    }

    #[test]
    fn create_price_from_dec_one_cent_test() {
        use super::Price;
        let price = Price::from_dec("kg", dec!(1));
        assert_eq!(price.get_value_formatted(), "0.01");
        assert_eq!(price.get_value(), dec!(0.01));
        assert_eq!(price.get_unit(), "kg");
    }

    #[test]
    fn create_price_from_str_one_cent_test() {
        use super::Price;
        let price = Price::from_str("kg", "1");
        assert_eq!(price.get_value_formatted(), "0.01");
        assert_eq!(price.get_value(), dec!(0.01));
        assert_eq!(price.get_unit(), "kg");
    }

    #[test]
    fn create_price_zero_cent_test() {
        use super::Price;
        let price = Price::new("kg", 0);
        assert_eq!(price.get_value_formatted(), "0.00");
        assert_eq!(price.get_value(), dec!(0.00));
        assert_eq!(price.get_unit(), "kg");
    }

    #[test]
    fn create_price_json_test() {
        use super::Price;

        let price = json!(
            {
                "unit": "kg",
                "value": 0.01
            }
        );

        let price_json: Price = serde_json::from_value(price).unwrap();
        assert_eq!(price_json.get_value_formatted(), "0.01");
        assert_eq!(price_json.get_value(), dec!(0.01));
        assert_eq!(price_json.get_unit(), "kg");

        let new_price = Price::new("kg", 1);
        let price_json = serde_json::to_value(new_price).unwrap();
        assert_eq!(
            price_json,
            json!(
                {
                    "unit": "kg",
                    "value": 0.01
                }
            )
        );
    }
}
