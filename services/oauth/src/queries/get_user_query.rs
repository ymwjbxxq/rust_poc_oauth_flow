use crate::dtos::login::get_user_request::GetUserRequest;
use crate::models::user::User;
use async_trait::async_trait;
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;
use shared::error::ApplicationError;
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

#[async_trait]
impl GetUserQuery for GetUser {
    async fn execute(&self, request: &GetUserRequest) -> Result<Option<User>, ApplicationError> {
        let res = self
            .client
            .get_item()
            .table_name(self.table_name.to_owned())
            .key(
                "client_id",
                AttributeValue::S(request.client_id.to_lowercase()),
            )
            .key(
                "user",
                AttributeValue::S(format!(
                    "{}#{}",
                    request.email.to_lowercase(),
                    request.password.to_lowercase()
                )),
            )
            .projection_expression("client_id, #user, is_consent, is_optin")
            .expression_attribute_names("#user", "user")
            .send()
            .await?;

        Ok(match res.item {
            None => None,
            Some(item) => Some(User::from_dynamodb(item)?),
        })
    }
}
