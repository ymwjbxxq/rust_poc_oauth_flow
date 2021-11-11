use crate::error::ApplicationError;
use aws_sdk_dynamodb::model::AttributeValue;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::utils::dynamodb::AttributeValuesExt;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct User {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub client_id: Option<String>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub email: Option<String>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub password: Option<String>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub family_name: Option<String>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub given_name: Option<String>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub is_consent: Option<bool>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub is_optin: Option<bool>,
}

impl User {
  pub fn to_dynamodb(&self) -> Result<HashMap<String, AttributeValue>, ApplicationError> {
    let mut retval = HashMap::new();
    retval.insert("client_id".to_owned(),   AttributeValue::S(self.client_id.as_ref().unwrap().clone()));
    retval.insert("email".to_owned(),       AttributeValue::S(self.email.as_ref().unwrap().clone()));
    retval.insert("password".to_owned(),    AttributeValue::S(self.password.as_ref().unwrap().clone()));
    retval.insert("family_name".to_owned(), AttributeValue::S(self.family_name.as_ref().unwrap().clone()));
    retval.insert("given_name".to_owned(),  AttributeValue::S(self.given_name.as_ref().unwrap().clone()));
    retval.insert("is_consent".to_owned(),  AttributeValue::Bool(self.is_consent.unwrap_or_default()));
    retval.insert("is_optin".to_owned(),    AttributeValue::Bool(self.is_optin.unwrap_or_default()));

    Ok(retval)
  }

  pub fn from_dynamodb(value: HashMap<String, AttributeValue>) -> Result<User, ApplicationError> {
    Ok(User {
      client_id:    value.get_string("client_id"),
      email:        value.get_string("email"),
      password:     value.get_string("password"),
      family_name:  value.get_string("family_name"),
      given_name:   value.get_string("given_name"),
      is_consent:   value.get_bool("is_consent"),
      is_optin:     value.get_bool("is_optin"),
    })
  }
}