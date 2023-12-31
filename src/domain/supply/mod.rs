pub mod price;
pub mod supply_id;
pub mod supply_validator;

use self::price::Price;
use self::supply_id::SupplyId;
use self::supply_validator::SupplyValidator;
use super::aggregate_root::AggregateRoot;
use super::entity::Entity;
use super::validation::notification::Notification;
use super::validation::validation_handler::ValidationHandler;
use super::validation::validator::Validator;
use chrono::DateTime;
use chrono::Utc;

/*
Input{
  name:"Cimento",
  price:Price
}
*/

#[derive(Clone, Debug)]
pub struct Supply<'a> {
    id: SupplyId,
    name: &'a str,
    price: Vec<Price>,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}

impl<'a> Supply<'a> {
    pub fn new(name: &'a str, price: Vec<Price>) -> Result<Self, Notification> {
        let input = Self {
            id: SupplyId::unique(),
            name,
            price,
            created_at: Utc::now(),
            updated_at: None,
        };
        input.self_validate()
    }

    pub fn with(id: SupplyId, name: &'a str, price: Vec<Price>) -> Result<Self, Notification> {
        let supply = Self {
            id,
            name,
            price,
            created_at: Utc::now(),
            updated_at: None,
        };
        supply.self_validate()
    }

    pub fn with_input(supply: &'a Supply) -> Self {
        Self {
            id: supply.id.to_owned(),
            name: supply.name,
            price: supply.price.to_owned(),
            created_at: supply.created_at.to_owned(),
            updated_at: supply.updated_at.to_owned(),
        }
    }

    pub fn update(
        &self,
        name: Option<&'a str>,
        price: Option<Vec<Price>>,
    ) -> Result<Self, Notification> {
        let supply = Self {
            id: self.id.to_owned(),
            name: name.unwrap_or(self.name),
            price: price.unwrap_or(self.price.to_owned()),
            created_at: self.created_at.to_owned(),
            updated_at: Some(Utc::now()),
        };
        supply.self_validate()
    }

    fn self_validate(self) -> Result<Self, Notification> {
        let mut notification = Notification::new();
        self.validate(&mut notification);
        if notification.has_errors() {
            return Err(notification);
        };
        Ok(self)
    }

    pub fn get_name(&self) -> &str {
        self.name
    }

    pub fn get_prices(&self) -> &Vec<Price> {
        &self.price
    }

    pub fn get_prices_to_string(&self) -> Vec<String> {
        self.price
            .iter()
            .map(|price| price.to_string())
            .collect::<Vec<String>>()
    }
    pub fn get_created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn get_updated_at(&self) -> Option<&DateTime<Utc>> {
        self.updated_at.as_ref()
    }
}

impl<'a> Entity for Supply<'a> {
    type Id = SupplyId;
    fn get_id(&self) -> &SupplyId {
        &self.id
    }

    fn validate(&self, handler: &mut dyn ValidationHandler) {
        let mut validator = SupplyValidator::new(self, handler);
        validator.validate();
    }
}

impl<'a> AggregateRoot for Supply<'a> {}

#[cfg(test)]
mod supply_tests {
    use super::*;
    use crate::domain::identifier::Identifier;
    use rust_decimal_macros::dec;
    use uuid::Uuid;

    #[test]
    fn create_a_valid_input() {
        let input = Supply::new("cimento", vec![Price::new("sc", 25000)]);
        assert!(input.is_ok());

        if let Ok(value) = input {
            assert_eq!(value.get_name(), "cimento");
            assert!(!value.get_id().get_value().is_empty());
            assert_eq!(value.get_prices().len(), 1);
            assert_eq!(value.get_prices().get(0).unwrap().get_unit(), "sc");
            assert_eq!(value.get_prices().get(0).unwrap().get_value(), dec!(250.00));
            assert_eq!(
                value.get_prices().get(0).unwrap().get_value_formatted(),
                "250.00"
            );
            assert!(!value.get_created_at().to_rfc3339().is_empty());
            assert!(value.get_updated_at().is_none());
        }
    }

    #[test]
    fn create_a_valid_input_with_id_param() {
        let supply = Supply::with(
            SupplyId::from_str("fake_id"),
            "cimento",
            vec![Price::new("sc", 25000)],
        );
        assert!(supply.is_ok());

        if let Ok(value) = supply {
            assert_eq!(value.get_name(), "cimento");
            assert!(!value.get_id().get_value().is_empty());
            assert_eq!(value.get_id().get_value(), "fake_id");
            assert_eq!(value.get_prices().len(), 1);
            assert_eq!(value.get_prices().get(0).unwrap().get_unit(), "sc");
            assert_eq!(value.get_prices().get(0).unwrap().get_value(), dec!(250.00));
            assert_eq!(
                value.get_prices().get(0).unwrap().get_value_formatted(),
                "250.00"
            );
            assert!(!value.get_created_at().to_rfc3339().is_empty());
            assert!(value.get_updated_at().is_none());
        }
    }

    #[test]
    fn create_a_valid_input_with_id_param_uuid() {
        let supply = Supply::with(
            SupplyId::from_uuid(Uuid::nil()),
            "cimento",
            vec![Price::new("sc", 25000)],
        );
        assert!(supply.is_ok());

        if let Ok(value) = supply {
            assert_eq!(value.get_name(), "cimento");
            assert!(!value.get_id().get_value().is_empty());
            assert_eq!(
                value.get_id().get_value(),
                "00000000-0000-0000-0000-000000000000"
            );
            assert_eq!(value.get_prices().len(), 1);
            assert_eq!(value.get_prices().get(0).unwrap().get_unit(), "sc");
            assert_eq!(value.get_prices().get(0).unwrap().get_value(), dec!(250.00));
            assert_eq!(
                value.get_prices().get(0).unwrap().get_value_formatted(),
                "250.00"
            );
            assert!(!value.get_created_at().to_rfc3339().is_empty());
            assert!(value.get_updated_at().is_none());
        }
    }

    #[test]
    fn create_an_invalid_input_with_name_empty() {
        let input = Supply::new("", vec![Price::new("sc", 25000)]);
        assert!(input.is_err());

        if let Err(error) = input {
            assert!(error.has_errors());
            assert_eq!(error.get_errors().len(), 2);
            assert_eq!(
                error.get_first_error().unwrap().to_string(),
                "'name' should not be empty"
            );
            assert_eq!(
                error.format_errors(),
                vec![
                    "'name' should not be empty",
                    "'name' must be between 1 and 255 characters"
                ]
            );
        }
    }

    #[test]
    fn create_an_invalid_input_with_price_empty() {
        let supply = Supply::new("cimento", vec![]);
        assert!(supply.is_err());

        if let Err(error) = supply {
            assert!(error.has_errors());
            assert_eq!(error.get_errors().len(), 1);
            assert_eq!(
                error.get_first_error().unwrap().to_string(),
                "'price' should not be empty"
            );
            assert_eq!(error.format_errors(), vec!["'price' should not be empty",]);
        }
    }

    #[test]
    fn create_an_invalid_input_with_id_empty() {
        let supply = Supply::with(
            SupplyId::from_str(""),
            "cimento",
            vec![Price::new("sc", 25000)],
        );
        assert!(supply.is_err());

        if let Err(error) = supply {
            assert!(error.has_errors());
            assert_eq!(error.get_errors().len(), 1);
            assert_eq!(
                error.get_first_error().unwrap().to_string(),
                "'id' should not be empty"
            );
            assert_eq!(error.format_errors(), vec!["'id' should not be empty",]);
        }
    }

    #[test]
    fn update_name_and_price_a_valid_input() {
        let supply = Supply::new("cimento", vec![Price::new("sc", 26000)]);

        if let Ok(value) = supply {
            let supply = value
                .update(
                    Some("cimento atualizado"),
                    Some(vec![Price::new("sc", 2500), Price::new("kg", 250)]),
                )
                .unwrap();

            assert_eq!(supply.get_name(), "cimento atualizado");
            assert!(!supply.get_id().get_value().is_empty());
            assert_eq!(supply.get_prices().len(), 2);
            assert_eq!(supply.get_prices().get(0).unwrap().get_unit(), "sc");
            assert_eq!(supply.get_prices().get(0).unwrap().get_value(), dec!(25.00));
            assert_eq!(
                supply.get_prices_to_string(),
                vec!["unit:sc value:25.00", "unit:kg value:2.50"]
            );
            assert_eq!(
                supply.get_prices().get(0).unwrap().get_value_formatted(),
                "25.00"
            );
            assert!(!supply.get_created_at().to_rfc3339().is_empty());
            assert!(supply.get_updated_at().is_some());
        }
    }

    #[test]
    fn create_a_valid_input_with_other_input() {
        let supply: Result<_, _> = Supply::new("cimento", vec![Price::new("sc", 26000)]);
        if let Ok(value) = supply {
            let other = Supply::with_input(&value);

            assert_eq!(other.get_name(), "cimento");
            assert!(!other.get_id().get_value().is_empty());
            assert_eq!(other.get_prices().len(), 1);
            assert_eq!(other.get_prices().get(0).unwrap().get_unit(), "sc");
            assert_eq!(other.get_prices().get(0).unwrap().get_value(), dec!(260.00));
            assert_eq!(other.get_prices_to_string(), vec!["unit:sc value:260.00"]);
            assert_eq!(
                other.get_prices().get(0).unwrap().get_value_formatted(),
                "260.00"
            );
            assert!(!other.get_created_at().to_rfc3339().is_empty());
            assert!(other.get_updated_at().is_none());
        }
    }
}
