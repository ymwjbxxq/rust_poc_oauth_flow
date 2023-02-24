use lambda_http::aws_lambda_events::query_map::QueryMap;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder as Builder;

#[derive(Builder)]
pub struct Page<'a> {
    #[builder(setter(into))]
    s3_client: &'a aws_sdk_s3::Client,

    #[builder(setter(into))]
    query_params: QueryMap,

    #[builder(setter(into))]
    bucket: String,
}

impl Page<'_> {
    pub async fn get_file_from_s3(&self, section: &str) -> Option<String> {
        let client_id = &self.query_params.first("client_id");

        if let Some(client_id) = client_id {
            let result = self
                .s3_client
                .get_object()
                .bucket(&self.bucket)
                .key(format!("{client_id}/{section}.json"))
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
                    return std::str::from_utf8(&bytes)
                        .and_then(|s| Ok(s.to_string()))
                        .ok()
                        .and_then(|json| Some(serde_json::from_str::<UI>(&json)))
                        .and_then(|ui| ui.ok())
                        .map(|page_type| page_type.page);
                }
            }
        }

        None
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UI {
    pub page: String,
}
