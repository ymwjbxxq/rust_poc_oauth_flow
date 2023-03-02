use async_trait::async_trait;
use typed_builder::TypedBuilder as Builder;

use crate::{queries::add_csrf_query::{AddCSRF, AddCSRFRequest, AddCSRFQuery}, error::ApplicationError};

#[async_trait]
pub trait LoginAppInitialisation: Send + Sync {
    fn redirect_path(&self) -> &str;
    fn oauth_authorize_uri(&self) -> &str;
    async fn add_csrf_query(&self, request: &AddCSRFRequest) -> Result<(), ApplicationError>;
}

#[derive(Debug, Builder)]
pub struct LoginAppClient {
    #[builder(setter(into))]
    pub redirect_path: String,

    #[builder(setter(into))]
    pub oauth_authorize_uri: String,

    #[builder(setter(into))]
    pub add_csrf_query: AddCSRF,
}

#[async_trait]
impl LoginAppInitialisation for LoginAppClient {
    fn redirect_path(&self) -> &str {
        &self.redirect_path
    }

    fn oauth_authorize_uri(&self) -> &str {
        &self.oauth_authorize_uri
    }

    async fn add_csrf_query(&self, request: &AddCSRFRequest) -> Result<(), ApplicationError> {
        self.add_csrf_query.execute(request).await
    }
}
