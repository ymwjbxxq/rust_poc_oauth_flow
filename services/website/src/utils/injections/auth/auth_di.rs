use crate::{
    models::csrf::CSRF,
    queries::{
        delete_csrf_query::{DeleteCSRF, DeleteCSRFQuery, DeleteCSRFRequest},
        get_csrf_query::{GetCSRF, GetCSRFQuery, GetCSRFRequest},
    },
};
use async_trait::async_trait;
use typed_builder::TypedBuilder as Builder;

#[async_trait]
pub trait AuthAppInitialisation: Send + Sync {
    fn oauth_token_uri(&self) -> &str;

    async fn get_csrf_query(&self, request: &GetCSRFRequest) -> anyhow::Result<Option<CSRF>>;

    async fn delete_csrf_query(&self, request: &DeleteCSRFRequest) -> anyhow::Result<()>;
}

#[derive(Debug, Builder)]
pub struct AuthAppClient {
    #[builder(setter(into))]
    pub oauth_token_uri: String,

    #[builder(setter(into))]
    pub get_csrf_query: GetCSRF,

    #[builder(setter(into))]
    pub delete_csrf_query: DeleteCSRF,
}

#[async_trait]
impl AuthAppInitialisation for AuthAppClient {
    async fn get_csrf_query(&self, request: &GetCSRFRequest) -> anyhow::Result<Option<CSRF>> {
        self.get_csrf_query.execute(request).await
    }

    async fn delete_csrf_query(&self, request: &DeleteCSRFRequest) -> anyhow::Result<()> {
        self.delete_csrf_query.execute(request).await
    }

    fn oauth_token_uri(&self) -> &str {
        &self.oauth_token_uri
    }
}
