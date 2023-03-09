use crate::{
    dtos::optin::update_optin_request::UpdateOptInRequest,
    queries::update_optin_query::{UpdateOptIn, UpdateOptInQuery},
};
use async_trait::async_trait;
use typed_builder::TypedBuilder as Builder;

#[async_trait]
pub trait PostAppInitialisation: Send + Sync {
    async fn query(&self, request: &UpdateOptInRequest) -> anyhow::Result<()>;
    fn redirect_path(&self) -> &str;
}

#[derive(Debug, Builder)]
pub struct PostAppClient {
    #[builder(setter(into))]
    pub query: UpdateOptIn,

    #[builder(setter(into))]
    pub redirect_path: String,
}

#[async_trait]
impl PostAppInitialisation for PostAppClient {
    async fn query(&self, request: &UpdateOptInRequest) -> anyhow::Result<()> {
        self.query.execute(request).await
    }

    fn redirect_path(&self) -> &str {
        &self.redirect_path
    }
}
