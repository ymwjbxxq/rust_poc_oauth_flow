use crate::dtos::consent::update_consent_request::UpdateConsentRequest;
use crate::error::ApplicationError;
use async_trait::async_trait;
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;
use typed_builder::TypedBuilder as Builder;


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
            .key("client_id",AttributeValue::S(request.client_id.to_lowercase()))
            .key("user", AttributeValue::S(request.user.to_lowercase()))
            .update_expression("set is_consent = :is_consent")
            .expression_attribute_values(":is_consent", AttributeValue::Bool(true))
            .send()
            .await?;
        println!("Consent updated");

        Ok(())
    }
}
