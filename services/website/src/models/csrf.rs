use aws_sdk_dynamodb::model::AttributeValue;
use serde::{Deserialize, Serialize};
use shared::{error::ApplicationError, utils::dynamodb::AttributeValuesExt};
use std::collections::HashMap;

#[derive(Clone, Default, Debug, Deserialize, PartialEq, Serialize)]
pub struct CSRF {
    pub client_id: String,

    pub sk: String,

    pub data: Option<String>,
}

impl CSRF {
    pub fn from_dynamodb(value: HashMap<String, AttributeValue>) -> Result<CSRF, ApplicationError> {
        Ok(Self {
            client_id: value.get_string("client_id").unwrap(),
            sk: value.get_string("sk").unwrap(),
            data: value.get_string("data"),
        })
    }
}
