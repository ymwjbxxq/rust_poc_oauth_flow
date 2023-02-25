use crate::error::ApplicationError;
use async_trait::async_trait;
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;
use std::collections::HashMap;
use typed_builder::TypedBuilder as Builder;

#[derive(Builder)]
pub struct UpdateConsentRequest {
    #[builder(setter(into))]
    pub client_id: String,

    #[builder(setter(into))]
    pub email: String,

    #[builder(setter(into))]
    pub is_consent: bool,
}

#[async_trait]
pub trait UpdateConsentQuery {
    async fn execute(&self, request: &UpdateConsentRequest) -> Result<(), ApplicationError>;
}

#[derive(Debug, Builder)]
pub struct UpdateConsent {
    #[builder(setter(into))]
    table_name: String,

    #[builder(setter(into))]
    client: Client,
}

#[async_trait]
impl UpdateConsentQuery for UpdateConsent {
    async fn execute(&self, request: &UpdateConsentRequest) -> Result<(), ApplicationError> {
        println!("Updating consent");
        self.client
            .update_item()
            .table_name(self.table_name.to_owned())
            .set_key(Some(HashMap::from([
                (
                    "client_id".to_owned(),
                    AttributeValue::S(request.client_id.to_lowercase()),
                ),
                (
                    "email".to_owned(),
                    AttributeValue::S(request.email.to_lowercase()),
                ),
            ])))
            .update_expression("set is_consent = :is_consent")
            .expression_attribute_values(":is_consent", AttributeValue::Bool(request.is_consent))
            .send()
            .await?;
        println!("Consent updated");

        Ok(())
    }
}
