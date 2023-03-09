use crate::dtos::token::get_user_request::GetUserRequest;
use crate::models::user::User;
use async_trait::async_trait;
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;
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
    async fn execute(&self, request: &GetUserRequest) -> anyhow::Result<Option<User>>;
}

#[async_trait]
impl GetUserQuery for GetUser {
    async fn execute(&self, request: &GetUserRequest) -> anyhow::Result<Option<User>> {
        let res = self
            .client
            .get_item()
            .table_name(self.table_name.to_owned())
            .key(
                "client_id",
                AttributeValue::S(request.client_id.to_lowercase()),
            )
            .key("user", AttributeValue::S(request.user.to_string()))
            .projection_expression("#user, is_consent, is_optin")
            .expression_attribute_names("#user", "user")
            .send()
            .await?;

        Ok(match res.item {
            None => None,
            Some(item) => Some(User::from_dynamodb(item)?),
        })
    }
}
