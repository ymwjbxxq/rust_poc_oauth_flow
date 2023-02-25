use crate::error::ApplicationError;
use crate::models::user::User;
use crate::utils::crypto::CriptoHelper;
use async_trait::async_trait;
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;
use std::collections::HashMap;
use typed_builder::TypedBuilder as Builder;

#[derive(Debug, Builder)]
pub struct GetUserRequest {
    #[builder(setter(into))]
    pub client_id: String,

    #[builder(setter(into))]
    pub email: String,
}

#[derive(Debug, Builder)]
pub struct GetUser {
    #[builder(setter(into))]
    table_name: String,

    #[builder(setter(into))]
    client: Client,
}

#[async_trait]
pub trait GetUserQuery {
    async fn execute(&self, request: &GetUserRequest) -> Result<Option<User>, ApplicationError>;
}

#[async_trait]
impl GetUserQuery for GetUser {
    async fn execute(&self, request: &GetUserRequest) -> Result<Option<User>, ApplicationError> {
        println!("Fetching user {:#?}", &request);
        let res = self
            .client
            .get_item()
            .table_name(self.table_name.to_owned())
            .set_key(Some(HashMap::from([
                (
                    "client_id".to_owned(),
                    AttributeValue::S(request.client_id.to_lowercase()),
                ),
                (
                    "email".to_owned(),
                    AttributeValue::S(CriptoHelper::to_sha256_string(request.email.to_lowercase())),
                ),
            ])))
            //.projection_expression("email, is_consent, is_optin")
            .send()
            .await?;

        Ok(match res.item {
            None => None,
            Some(item) => Some(User::from_dynamodb(item)?),
        })
    }
}
