use crate::dtos::oauth::signup::singup_request::SignUpRequest;
use crate::error::ApplicationError;
use async_trait::async_trait;
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::model::AttributeValue;
use typed_builder::TypedBuilder as Builder;

#[derive(Debug, Builder)]
pub struct AddUser {
    #[builder(setter(into))]
    table_name: String,

    #[builder(setter(into))]
    client: Client,
}

#[async_trait]
pub trait AddQuery {
    async fn execute(&self, product: &SignUpRequest) -> Result<(), ApplicationError>;
}

#[async_trait]
impl AddQuery for AddUser {
    async fn execute(&self, request: &SignUpRequest) -> Result<(), ApplicationError> {
        println!("Adding user");
        let res = self
            .client
            .put_item()
            .table_name(self.table_name.to_owned())
            .item(
                "client_id",
                AttributeValue::S(request.client_id.clone().unwrap().to_lowercase()),
            )
            .item(
                "user",
                AttributeValue::S(format!("{}#{}", request.email.to_lowercase(), request.password.to_lowercase())),
            )
            .item(
                "family_name",
                AttributeValue::S(request.family_name.to_owned()),
            )
            .item(
                "given_name",
                AttributeValue::S(request.given_name.to_owned()),
            )
            .item(
                "is_consent",
                AttributeValue::Bool(request.is_consent.unwrap_or_default()),
            )
            .item(
                "is_optin",
                AttributeValue::Bool(request.is_optin.unwrap_or_default()),
            )
            .send()
            .await?;
        println!("User added {res:?}");

        Ok(())
    }
}
