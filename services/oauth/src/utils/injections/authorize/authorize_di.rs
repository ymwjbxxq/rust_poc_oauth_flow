use async_trait::async_trait;
use typed_builder::TypedBuilder as Builder;

use crate::{
    error::ApplicationError,
    models::csrf::CSRF,
    queries::get_csrf_query::{GetCSRF, GetCSRFQuery, GetCSRFRequest},
};

#[async_trait]
pub trait AuthorizeAppInitialisation: Send + Sync {
    async fn get_csrf_query(
        &self,
        request: &GetCSRFRequest,
    ) -> Result<Option<CSRF>, ApplicationError>;
    fn oauth_authorize_login_path(&self) -> &str;

    fn oauth_custom_optin_path(&self) -> &str;

    fn oauth_custom_consent_path(&self) -> &str;
}

#[derive(Debug, Builder)]
pub struct AuthorizeAppClient {
    #[builder(setter(into))]
    pub get_csrf_query: GetCSRF,

    #[builder(setter(into))]
    pub oauth_authorize_login_path: String,

    #[builder(setter(into))]
    pub oauth_custom_optin_path: String,

    #[builder(setter(into))]
    pub oauth_custom_consent_path: String,
}

#[async_trait]
impl AuthorizeAppInitialisation for AuthorizeAppClient {
    async fn get_csrf_query(
        &self,
        request: &GetCSRFRequest,
    ) -> Result<Option<CSRF>, ApplicationError> {
        self.get_csrf_query.execute(request).await
    }

    fn oauth_authorize_login_path(&self) -> &str {
        &self.oauth_authorize_login_path
    }

    fn oauth_custom_optin_path(&self) -> &str {
        &self.oauth_custom_optin_path
    }

    fn oauth_custom_consent_path(&self) -> &str {
        &self.oauth_custom_consent_path
    }
}
