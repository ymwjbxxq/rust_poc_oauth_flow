use crate::{
    error::ApplicationError,
    queries::update_consent_query::{UpdateConsent, UpdateConsentQuery}, dtos::consent::update_consent_request::UpdateConsentRequest,
};
use async_trait::async_trait;
use typed_builder::TypedBuilder as Builder;

#[async_trait]
pub trait PostAppInitialisation: Send + Sync {
    async fn query(&self, request: &UpdateConsentRequest) -> Result<(), ApplicationError>;
    fn redirect_path(&self) -> &str;
}

#[derive(Debug, Builder)]
pub struct PostAppClient {
    #[builder(setter(into))]
    pub query: UpdateConsent,

    #[builder(setter(into))]
    pub redirect_path: String,
}

#[async_trait]
impl PostAppInitialisation for PostAppClient {
    async fn query(&self, request: &UpdateConsentRequest) -> Result<(), ApplicationError> {
        self.query.execute(request).await
    }

    fn redirect_path(&self) -> &str {
        &self.redirect_path
    }
}
