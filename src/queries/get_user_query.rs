use crate::error::ApplicationError;
use crate::models::user::User;
use crate::utils::crypto::CriptoHelper;
use async_trait::async_trait;
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;
use std::collections::HashMap;

#[async_trait]
pub trait GetUserQuery {
    fn new(client: &Client) -> Self;
    async fn execute(&self, client_id: &str, email: &str)
        -> Result<Option<User>, ApplicationError>;
}

#[derive(Debug)]
pub struct LoginQuery {
    table_name: String,
    client: Client,
}

#[async_trait]
impl GetUserQuery for LoginQuery {
    fn new(client: &Client) -> Self {
        let table_name = std::env::var("TABLE_NAME").expect("TABLE_NAME must be set");
        Self {
            table_name,
            client: client.clone(),
        }
    }

    async fn execute(
        &self,
        client_id: &str,
        email: &str,
    ) -> Result<Option<User>, ApplicationError> {
        println!("Fetching user for app {client_id} with {email}");
        let res = self
            .client
            .get_item()
            .table_name(self.table_name.to_owned())
            .set_key(Some(HashMap::from([
                (
                    "client_id".to_owned(),
                    AttributeValue::S(client_id.to_owned()),
                ),
                (
                    "email".to_owned(),
                    AttributeValue::S(CriptoHelper::to_sha256_string(email)),
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
