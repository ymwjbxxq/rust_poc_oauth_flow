use crate::error::ApplicationError;
use crate::models::user::User;
use async_trait::async_trait;
use aws_sdk_dynamodb::Client;
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
    async fn execute(&self, product: &User) -> Result<(), ApplicationError>;
}

#[async_trait]
impl AddQuery for AddUser {
    async fn execute(&self, request: &User) -> Result<(), ApplicationError> {
        println!("Adding user");
        let res = self
            .client
            .put_item()
            .table_name(self.table_name.to_owned())
            .set_item(Some(request.to_dynamodb()?))
            .send()
            .await?;
        println!("User added {res:?}");

        Ok(())
    }
}
