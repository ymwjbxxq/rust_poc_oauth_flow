use async_trait::async_trait;
use typed_builder::TypedBuilder as Builder;

#[async_trait]
pub trait LoginAppInitialisation: Send + Sync {
    fn redirect_path(&self) -> &str;
    fn oauth_authorize_uri(&self) -> &str;
}

#[derive(Debug, Builder)]
pub struct LoginAppClient {
    #[builder(setter(into))]
    pub redirect_path: String,

    #[builder(setter(into))]
    pub oauth_authorize_uri: String,
}

#[async_trait]
impl LoginAppInitialisation for LoginAppClient {
    fn redirect_path(&self) -> &str {
        &self.redirect_path
    }

    fn oauth_authorize_uri(&self) -> &str {
        &self.oauth_authorize_uri
    }
}