use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;
use async_trait::async_trait;
use std::collections::HashMap;
use crate::error::ApplicationError;

pub struct UpdateOptInCommand {
  pub client_id: String,
  pub email: String,
  pub is_optin: bool,
}

#[async_trait]
pub trait UpdateOptIn {
    fn new(client: &Client) -> Self;
    async fn execute(&self, command: UpdateOptInCommand) -> Result<(), ApplicationError>;
}

#[derive(Debug)]
pub struct UpdateOptInQuery {
  table_name: String,
  client: Client,
}

#[async_trait]
impl UpdateOptIn for UpdateOptInQuery {
  fn new(client: &Client) -> Self {
    let table_name = std::env::var("TABLE_NAME").expect("TABLE_NAME must be set");
    Self { 
      table_name,
      client: client.clone(),
    }
  }

  async fn execute(&self, command: UpdateOptInCommand) -> Result<(), ApplicationError> {
    log::info!("Updating optin");
    self.client
      .update_item()
      .table_name(self.table_name.to_owned())
      .set_key(Some(HashMap::from([
          ("client_id".to_owned(), AttributeValue::S(command.client_id)),
          ("email".to_owned(), AttributeValue::S(command.email)),
        ])))
      .update_expression("set is_optin = :is_optin")
      .expression_attribute_values(":is_optin", AttributeValue::Bool(command.is_optin))
      .send()
      .await?;
    log::info!("Optin updated");

    Ok(())
  }
}
