use async_trait::async_trait;
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;
use chrono::{Duration, Utc};
use shared::error::ApplicationError;
use typed_builder::TypedBuilder as Builder;

#[derive(Debug, Default, Builder)]
pub struct AddCSRFRequest {
    #[builder(default, setter(into))]
    pub client_id: String,

    #[builder(default, setter(into))]
    pub sk: String,

    #[builder(default, setter(into))]
    pub data: Option<String>,
}

#[derive(Debug, Builder)]
pub struct AddCSRF {
    #[builder(setter(into))]
    table_name: String,

    #[builder(setter(into))]
    client: Client,
}

#[async_trait]
pub trait AddCSRFQuery {
    async fn execute(&self, product: &AddCSRFRequest) -> Result<(), ApplicationError>;
}

#[async_trait]
impl AddCSRFQuery for AddCSRF {
    async fn execute(&self, request: &AddCSRFRequest) -> Result<(), ApplicationError> {
        let ttl = (Utc::now() + Duration::minutes(1)).timestamp();
        let mut query = self
            .client
            .put_item()
            .table_name(self.table_name.to_owned())
            .item(
                "client_id",
                AttributeValue::S(request.client_id.to_lowercase()),
            )
            .item("sk", AttributeValue::S(request.sk.to_owned()))
            .item("ttl", AttributeValue::N(format!("{ttl:?}")));

        if let Some(data) = &request.data {
            query = query.item("data", AttributeValue::S(data.to_owned()));
        }

        query.send().await?;

        Ok(())
    }
}
