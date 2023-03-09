use aws_sdk_dynamodb::model::AttributeValue;
use serde::{Deserialize, Serialize};
use shared::utils::dynamodb::AttributeValuesExt;
use std::collections::HashMap;

#[derive(Clone, Default, Debug, Deserialize, PartialEq, Serialize)]
pub struct User {
    pub user: String,

    pub is_consent: bool,

    pub is_optin: bool,
}

impl User {
    pub fn from_dynamodb(value: HashMap<String, AttributeValue>) -> anyhow::Result<User> {
        Ok(User {
            user: value.get_string("user").unwrap(),
            is_consent: value.get_bool("is_consent").unwrap_or_default(),
            is_optin: value.get_bool("is_optin").unwrap_or_default(),
        })
    }
}
