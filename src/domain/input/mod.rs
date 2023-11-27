pub mod input_id;
pub mod input_validator;
pub mod price;

use self::input_id::InputId;
use self::input_validator::InputValidator;
use self::price::Price;
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
pub struct Input<'a> {
    id: InputId,
    name: &'a str,
    price: Vec<Price>,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}

impl<'a> Input<'a> {
    pub fn new(name: &'a str, price: Vec<Price>) -> Result<Self, Notification> {
        let input = Self {
            id: InputId::unique(),
            name,
            price,
            created_at: Utc::now(),
            updated_at: None,
        };
        input.self_validate()
    }

    pub fn with(id: InputId, name: &'a str, price: Vec<Price>) -> Result<Self, Notification> {
        let input = Self {
            id,
            name,
            price,
            created_at: Utc::now(),
            updated_at: None,
        };
        input.self_validate()
    }

    pub fn with_input(input: &'a Input) -> Self {
        Self {
            id: input.id.to_owned(),
            name: input.name,
            price: input.price.to_owned(),
            created_at: input.created_at.to_owned(),
            updated_at: input.updated_at.to_owned(),
        }
    }

    pub fn update(
        &self,
        name: Option<&'a str>,
        price: Option<Vec<Price>>,
    ) -> Result<Self, Notification> {
        let input = Self {
            id: self.id.to_owned(),
            name: name.unwrap_or(self.name),
            price: price.unwrap_or(self.price.to_owned()),
            created_at: self.created_at.to_owned(),
            updated_at: Some(Utc::now()),
        };
        input.self_validate()
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

impl<'a> Entity for Input<'a> {
    type Id = InputId;
    fn get_id(&self) -> &InputId {
        &self.id
    }

    fn validate(&self, handler: &mut dyn ValidationHandler) {
        let mut validator = InputValidator::new(self, handler);
        validator.validate();
    }
}

impl<'a> AggregateRoot for Input<'a> {}

#[cfg(test)]
mod input_tests {
    use super::*;
    use crate::domain::identifier::Identifier;
    use uuid::Uuid;

    #[test]
    fn create_a_valid_input() {
        let input = Input::new("cimento", vec![Price::new("sc", 25000)]);
        assert!(input.is_ok());

        if let Ok(value) = input {
            assert_eq!(value.get_name(), "cimento");
            assert!(!value.get_id().get_value().is_empty());
            assert_eq!(value.get_prices().len(), 1);
            assert_eq!(value.get_prices().get(0).unwrap().get_unit(), "sc");
            assert_eq!(value.get_prices().get(0).unwrap().get_value(), 25000);
            assert_eq!(
                value.get_prices().get(0).unwrap().get_value_formatted(),
                "250,00"
            );
            assert!(!value.get_created_at().to_rfc3339().is_empty());
            assert!(value.get_updated_at().is_none());
        }
    }

    #[test]
    fn create_a_valid_input_with_id_param() {
        let input = Input::with(
            InputId::from_str("fake_id"),
            "cimento",
            vec![Price::new("sc", 25000)],
        );
        assert!(input.is_ok());

        if let Ok(value) = input {
            assert_eq!(value.get_name(), "cimento");
            assert!(!value.get_id().get_value().is_empty());
            assert_eq!(value.get_id().get_value(), "fake_id");
            assert_eq!(value.get_prices().len(), 1);
            assert_eq!(value.get_prices().get(0).unwrap().get_unit(), "sc");
            assert_eq!(value.get_prices().get(0).unwrap().get_value(), 25000);
            assert_eq!(
                value.get_prices().get(0).unwrap().get_value_formatted(),
                "250,00"
            );
            assert!(!value.get_created_at().to_rfc3339().is_empty());
            assert!(value.get_updated_at().is_none());
        }
    }

    #[test]
    fn create_a_valid_input_with_id_param_uuid() {
        let input = Input::with(
            InputId::from_uuid(Uuid::nil()),
            "cimento",
            vec![Price::new("sc", 25000)],
        );
        assert!(input.is_ok());

        if let Ok(value) = input {
            assert_eq!(value.get_name(), "cimento");
            assert!(!value.get_id().get_value().is_empty());
            assert_eq!(
                value.get_id().get_value(),
                "00000000-0000-0000-0000-000000000000"
            );
            assert_eq!(value.get_prices().len(), 1);
            assert_eq!(value.get_prices().get(0).unwrap().get_unit(), "sc");
            assert_eq!(value.get_prices().get(0).unwrap().get_value(), 25000);
            assert_eq!(
                value.get_prices().get(0).unwrap().get_value_formatted(),
                "250,00"
            );
            assert!(!value.get_created_at().to_rfc3339().is_empty());
            assert!(value.get_updated_at().is_none());
        }
    }

    #[test]
    fn create_an_invalid_input_with_name_empty() {
        let input = Input::new("", vec![Price::new("sc", 25000)]);
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
        let input = Input::new("cimento", vec![]);
        assert!(input.is_err());

        if let Err(error) = input {
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
        let input = Input::with(
            InputId::from_str(""),
            "cimento",
            vec![Price::new("sc", 25000)],
        );
        assert!(input.is_err());

        if let Err(error) = input {
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
        let input = Input::new("cimento", vec![Price::new("sc", 26000)]);

        if let Ok(value) = input {
            let input = value
                .update(
                    Some("cimento atualizado"),
                    Some(vec![Price::new("sc", 2500), Price::new("kg", 250)]),
                )
                .unwrap();

            assert_eq!(input.get_name(), "cimento atualizado");
            assert!(!input.get_id().get_value().is_empty());
            assert_eq!(input.get_prices().len(), 2);
            assert_eq!(input.get_prices().get(0).unwrap().get_unit(), "sc");
            assert_eq!(input.get_prices().get(0).unwrap().get_value(), 2500);
            assert_eq!(
                input.get_prices_to_string(),
                vec!["unit:sc value:2500", "unit:kg value:250"]
            );
            assert_eq!(
                input.get_prices().get(0).unwrap().get_value_formatted(),
                "25,00"
            );
            assert!(!input.get_created_at().to_rfc3339().is_empty());
            assert!(input.get_updated_at().is_some());
        }
    }

    #[test]
    fn create_a_valid_input_with_other_input() {
        let input = Input::new("cimento", vec![Price::new("sc", 26000)]);
        if let Ok(value) = input {
            let other = Input::with_input(&value);

            assert_eq!(other.get_name(), "cimento");
            assert!(!other.get_id().get_value().is_empty());
            assert_eq!(other.get_prices().len(), 1);
            assert_eq!(other.get_prices().get(0).unwrap().get_unit(), "sc");
            assert_eq!(other.get_prices().get(0).unwrap().get_value(), 26000);
            assert_eq!(other.get_prices_to_string(), vec!["unit:sc value:26000"]);
            assert_eq!(
                other.get_prices().get(0).unwrap().get_value_formatted(),
                "260,00"
            );
            assert!(!other.get_created_at().to_rfc3339().is_empty());
            assert!(other.get_updated_at().is_none());
        }
    }
}
