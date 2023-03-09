use crate::models::csrf::CSRF;
use async_trait::async_trait;
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;
use typed_builder::TypedBuilder as Builder;

#[derive(Debug, Default, Builder)]
pub struct GetCSRFRequest {
    #[builder(default, setter(into))]
    pub client_id: String,

    #[builder(default, setter(into))]
    pub sk: String,
}

#[derive(Debug, Builder)]
pub struct GetCSRF {
    #[builder(setter(into))]
    table_name: String,

    #[builder(setter(into))]
    client: Client,
}

#[async_trait]
pub trait GetCSRFQuery {
    async fn execute(&self, request: &GetCSRFRequest) -> anyhow::Result<Option<CSRF>>;
}

#[async_trait]
impl GetCSRFQuery for GetCSRF {
    async fn execute(&self, request: &GetCSRFRequest) -> anyhow::Result<Option<CSRF>> {
        let res = self
            .client
            .get_item()
            .table_name(self.table_name.to_owned())
            .key(
                "client_id",
                AttributeValue::S(request.client_id.to_lowercase()),
            )
            .key("sk", AttributeValue::S(request.sk.to_owned()))
            .projection_expression("client_id, #data, sk")
            .expression_attribute_names("#data", "data")
            .send()
            .await?;

        Ok(match res.item {
            None => None,
            Some(item) => Some(CSRF::from_dynamodb(item)?),
        })
    }
}
