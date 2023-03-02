use async_trait::async_trait;
use typed_builder::TypedBuilder as Builder;

use crate::{
    error::ApplicationError,
    models::csrf::CSRF,
    queries::get_csrf_query::{GetCSRF, GetCSRFQuery, GetCSRFRequest},
};

#[async_trait]
pub trait ToeknAppInitialisation: Send + Sync {
    async fn get_csrf_query(
        &self,
        request: &GetCSRFRequest,
    ) -> Result<Option<CSRF>, ApplicationError>;
}

#[derive(Debug, Builder)]
pub struct ToeknAppClient {
    #[builder(setter(into))]
    pub get_csrf_query: GetCSRF,
}

#[async_trait]
impl ToeknAppInitialisation for ToeknAppClient {
    async fn get_csrf_query(
        &self,
        request: &GetCSRFRequest,
    ) -> Result<Option<CSRF>, ApplicationError> {
        self.get_csrf_query.execute(request).await
    }
}
