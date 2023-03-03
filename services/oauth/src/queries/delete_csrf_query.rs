use async_trait::async_trait;
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;
use shared::error::ApplicationError;
use typed_builder::TypedBuilder as Builder;

#[derive(Debug, Default, Builder)]
pub struct DeleteCSRFRequest {
    #[builder(default, setter(into))]
    pub client_id: String,

    #[builder(default, setter(into))]
    pub sk: String,
}

#[derive(Debug, Builder)]
pub struct DeleteCSRF {
    #[builder(setter(into))]
    table_name: String,

    #[builder(setter(into))]
    client: Client,
}

#[async_trait]
pub trait DeleteCSRFQuery {
    async fn execute(&self, request: &DeleteCSRFRequest) -> Result<(), ApplicationError>;
}

#[async_trait]
impl DeleteCSRFQuery for DeleteCSRF {
    async fn execute(&self, request: &DeleteCSRFRequest) -> Result<(), ApplicationError> {
        self.client
            .delete_item()
            .table_name(self.table_name.to_owned())
            .key(
                "client_id",
                AttributeValue::S(request.client_id.to_lowercase()),
            )
            .key("sk", AttributeValue::S(request.sk.to_owned()))
            .send()
            .await?;

        Ok(())
    }
}
