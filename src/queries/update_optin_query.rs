use crate::dtos::oauth::optin::update_optin_request::UpdateOptInRequest;
use crate::error::ApplicationError;
use async_trait::async_trait;
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;
use std::collections::HashMap;
use typed_builder::TypedBuilder as Builder;

#[async_trait]
pub trait UpdateOptInQuery {
    async fn execute(&self, request: &UpdateOptInRequest) -> Result<(), ApplicationError>;
}

#[derive(Debug, Builder)]
pub struct UpdateOptIn {
    #[builder(setter(into))]
    table_name: String,

    #[builder(setter(into))]
    client: Client,
}

#[async_trait]
impl UpdateOptInQuery for UpdateOptIn {
    async fn execute(&self, request: &UpdateOptInRequest) -> Result<(), ApplicationError> {
        println!("Updating optin");
        self.client
            .update_item()
            .table_name(self.table_name.to_owned())
            .set_key(Some(HashMap::from([
                (
                    "client_id".to_owned(),
                    AttributeValue::S(request.client_id.to_lowercase()),
                ),
                (
                    "user".to_owned(),
                    AttributeValue::S(request.user.to_lowercase()),
                ),
            ])))
            .update_expression("set is_optin = :is_optin")
            .expression_attribute_values(":is_optin", AttributeValue::Bool(true))
            .send()
            .await?;
        println!("Optin updated");

        Ok(())
    }
}
