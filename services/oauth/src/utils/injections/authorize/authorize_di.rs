use crate::{
    models::csrf::CSRF,
    queries::csrf::get_csrf_query::{GetCSRF, GetCSRFQuery, GetCSRFRequest},
};
use async_trait::async_trait;
use typed_builder::TypedBuilder as Builder;

#[async_trait]
pub trait AuthorizeAppInitialisation: Send + Sync {
    async fn get_csrf_query(&self, request: &GetCSRFRequest) -> anyhow::Result<Option<CSRF>>;
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
    async fn get_csrf_query(&self, request: &GetCSRFRequest) -> anyhow::Result<Option<CSRF>> {
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
