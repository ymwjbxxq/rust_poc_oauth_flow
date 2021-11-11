use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;
use async_trait::async_trait;
use std::collections::HashMap;
use crate::error::ApplicationError;

pub struct UpdateConsentCommand {
  pub client_id: String,
  pub email: String,
  pub is_consent: bool,
}

#[async_trait]
pub trait UpdateConsent {
    fn new(client: &Client) -> Self;
    async fn execute(&self, command: UpdateConsentCommand) -> Result<(), ApplicationError>;
}

#[derive(Debug)]
pub struct UpdateConsentQuery {
  table_name: String,
  client: Client,
}

#[async_trait]
impl UpdateConsent for UpdateConsentQuery {
  fn new(client: &Client) -> Self {
    let table_name = std::env::var("TABLE_NAME").expect("TABLE_NAME must be set");
    Self { 
      table_name,
      client: client.clone(),
    }
  }

  async fn execute(&self, command: UpdateConsentCommand) -> Result<(), ApplicationError> {
    log::info!("Updating consent");
    self.client
      .update_item()
      .table_name(self.table_name.to_owned())
      .set_key(Some(HashMap::from([
          ("client_id".to_owned(), AttributeValue::S(command.client_id)),
          ("email".to_owned(), AttributeValue::S(command.email)),
        ])))
      .update_expression("set is_consent = :is_consent")
      .expression_attribute_values(":is_consent", AttributeValue::Bool(command.is_consent))
      .send()
      .await?;
    log::info!("Consent updated");

    Ok(())
  }
}
