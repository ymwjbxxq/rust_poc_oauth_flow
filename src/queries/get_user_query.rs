use crate::dtos::oauth::login::get_user_request::GetUserRequest;
use crate::error::ApplicationError;
use crate::models::user::User;
use crate::utils::crypto::CriptoHelper;
use async_trait::async_trait;
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;
use std::collections::HashMap;
use typed_builder::TypedBuilder as Builder;

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

//TODO use password to login
#[async_trait]
impl GetUserQuery for GetUser {
    async fn execute(&self, request: &GetUserRequest) -> Result<Option<User>, ApplicationError> {
        println!("Fetching user {:#?}", &request);
        let user = format!(
            "{}#{}",
            CriptoHelper::to_sha256_string(request.email.to_lowercase()),
            CriptoHelper::to_sha256_string(request.password.to_lowercase())
        );
        let res = self
            .client
            .get_item()
            .table_name(self.table_name.to_owned())
            .set_key(Some(HashMap::from([
                (
                    "client_id".to_owned(),
                    AttributeValue::S(request.client_id.to_lowercase()),
                ),
                ("user".to_owned(), AttributeValue::S(user)),
            ])))
            .projection_expression("client_id, user, is_consent, is_optin")
            .send()
            .await?;

        Ok(match res.item {
            None => None,
            Some(item) => Some(User::from_dynamodb(item)?),
        })
    }
}
