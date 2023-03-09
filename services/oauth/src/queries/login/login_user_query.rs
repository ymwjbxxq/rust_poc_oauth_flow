use crate::dtos::login::login_user_request::LoginUserRequest;
use crate::models::user::User;
use async_trait::async_trait;
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;
use typed_builder::TypedBuilder as Builder;

#[derive(Debug, Builder)]
pub struct LoginUser {
    #[builder(setter(into))]
    table_name: String,

    #[builder(setter(into))]
    client: Client,
}

#[async_trait]
pub trait LoginUserQuery {
    async fn execute(&self, request: &LoginUserRequest) -> anyhow::Result<Option<User>>;
}

#[async_trait]
impl LoginUserQuery for LoginUser {
    async fn execute(&self, request: &LoginUserRequest) -> anyhow::Result<Option<User>> {
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
                    "{}####{}",
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
