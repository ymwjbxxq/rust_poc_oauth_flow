use crate::{
    error::ApplicationError,
    queries::get_page_query::{Page, PageQuery, PageRequest},
};
use async_trait::async_trait;
use typed_builder::TypedBuilder as Builder;

#[async_trait]
pub trait GetAppInitialisation: Send + Sync {
    async fn query(&self, request: &PageRequest) -> Result<Option<String>, ApplicationError>;
}

#[derive(Debug, Builder)]
pub struct GetAppClient {
    #[builder(setter(into))]
    pub query: Page,
}

#[async_trait]
impl GetAppInitialisation for GetAppClient {
    async fn query(&self, request: &PageRequest) -> Result<Option<String>, ApplicationError> {
        self.query.execute(request).await
    }
}
