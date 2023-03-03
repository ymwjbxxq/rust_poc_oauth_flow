use crate::{error::ApplicationError, dtos::load_page::page_request::PageRequest};
use async_trait::async_trait;
use aws_sdk_s3::Client;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder as Builder;

#[async_trait]
pub trait PageQuery {
    async fn execute(&self, request: &PageRequest) -> Result<Option<String>, ApplicationError>;
}

#[derive(Debug, Builder)]
pub struct Page {
    #[builder(setter(into))]
    bucket_name: String,

    #[builder(setter(into))]
    pub page_name: String,

    #[builder(setter(into))]
    client: Client,
}

#[async_trait]
impl PageQuery for Page {
    async fn execute(&self, request: &PageRequest) -> Result<Option<String>, ApplicationError> {
        let result = self
            .client
            .get_object()
            .bucket(&self.bucket_name)
            .key(format!("{}/{}.json", request.client_id, &self.page_name))
            .response_content_type("application/json")
            .send()
            .await;
        if let Ok(result) = result {
            let bytes = result
                .body
                .collect()
                .await
                .ok()
                .map(|body| body.into_bytes());
            if let Some(bytes) = bytes {
                return Ok(std::str::from_utf8(&bytes)
                    .and_then(|s| Ok(s.to_string()))
                    .ok()
                    .and_then(|json| Some(serde_json::from_str::<UI>(&json)))
                    .and_then(|ui| ui.ok())
                    .map(|page_type| page_type.page));
            }
        }
        Ok(None)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UI {
    pub page: String,
}
