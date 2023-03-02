use async_trait::async_trait;
use typed_builder::TypedBuilder as Builder;

use crate::{error::ApplicationError, models::csrf::CSRF, queries::get_csrf_query::{GetCSRFRequest, GetCSRF, GetCSRFQuery}};

#[async_trait]
pub trait AuthAppInitialisation: Send + Sync {
    fn oauth_token_uri(&self) -> &str;

    async fn get_csrf_query(
        &self,
        request: &GetCSRFRequest,
    ) -> Result<Option<CSRF>, ApplicationError>;
}

#[derive(Debug, Builder)]
pub struct AuthAppClient {
    #[builder(setter(into))]
    pub oauth_token_uri: String,

    #[builder(setter(into))]
    pub get_csrf_query: GetCSRF,
}

#[async_trait]
impl AuthAppInitialisation for AuthAppClient {
    async fn get_csrf_query(
        &self,
        request: &GetCSRFRequest,
    ) -> Result<Option<CSRF>, ApplicationError> {
        self.get_csrf_query.execute(request).await
    }

    fn oauth_token_uri(&self) -> &str {
        &self.oauth_token_uri
    }
}
