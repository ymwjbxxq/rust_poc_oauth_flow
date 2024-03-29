use crate::{
    dtos::token::get_key_request::GetKeyRequest,
    queries::token::get_key::{GetKey, GetKeyQuery},
};
use async_trait::async_trait;
use typed_builder::TypedBuilder as Builder;

#[async_trait]
pub trait JwksAppInitialisation: Send + Sync {
    async fn get_key_query(&self, request: &GetKeyRequest) -> anyhow::Result<Option<String>>;
}

#[derive(Debug, Builder)]
pub struct JwksAppClient {
    #[builder(setter(into))]
    pub get_key_query: GetKey,
}

#[async_trait]
impl JwksAppInitialisation for JwksAppClient {
    async fn get_key_query(&self, request: &GetKeyRequest) -> anyhow::Result<Option<String>> {
        self.get_key_query.execute(request).await
    }
}
