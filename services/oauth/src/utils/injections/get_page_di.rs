use crate::{
    error::ApplicationError,
    queries::get_page_query::{Page, PageQuery}, dtos::load_page::page_request::PageRequest,
};
use async_trait::async_trait;
use typed_builder::TypedBuilder as Builder;

#[async_trait]
pub trait GetPageAppInitialisation: Send + Sync {
    async fn query(&self, request: &PageRequest) -> Result<Option<String>, ApplicationError>;
}

#[derive(Debug, Builder)]
pub struct GetPageAppClient {
    #[builder(setter(into))]
    pub query: Page,
}

#[async_trait]
impl GetPageAppInitialisation for GetPageAppClient {
    async fn query(&self, request: &PageRequest) -> Result<Option<String>, ApplicationError> {
        self.query.execute(request).await
    }
}
