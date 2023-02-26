use async_trait::async_trait;
use typed_builder::TypedBuilder as Builder;

#[async_trait]
pub trait AuthAppInitialisation: Send + Sync {
    fn oauth_token_uri(&self) -> &str;
}

#[derive(Debug, Builder)]
pub struct AuthAppClient {
    #[builder(setter(into))]
    pub oauth_token_uri: String,
}

#[async_trait]
impl AuthAppInitialisation for AuthAppClient {
    fn oauth_token_uri(&self) -> &str {
        &self.oauth_token_uri
    }
}