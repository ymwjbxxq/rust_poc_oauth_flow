use crate::error::ApplicationError;
use crate::models::user::User;
use aws_sdk_dynamodb::Client;
use async_trait::async_trait;

#[async_trait]
pub trait AddQuery {
    fn new(client: &Client) -> Self;
    async fn execute(&self, product: &User) -> Result<(), ApplicationError>;
}

#[derive(Debug)]
pub struct AddUser {
  table_name: String,
  client: Client,
}

#[async_trait]
impl AddQuery for AddUser {
  fn new(client: &Client) -> Self {
    let table_name = std::env::var("TABLE_NAME").expect("TABLE_NAME must be set");
    Self { 
      table_name,
      client: client.clone(),
    }
  }

  async fn execute(&self, request: &User) -> Result<(), ApplicationError> {
    log::info!("Adding user");
    let res = self.client
      .put_item()
      .table_name(self.table_name.to_owned())
      .set_item(Some(request.to_dynamodb()?))
      .send()
      .await?;
     log::info!("User added {:?}", res);

    Ok(())
  }
}
