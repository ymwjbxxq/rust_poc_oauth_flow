use crate::{
    error::ApplicationError, queries::add_user_query::{AddUser, AddQuery}, models::user::User
};
use async_trait::async_trait;
use typed_builder::TypedBuilder as Builder;

#[async_trait]
pub trait PostAppInitialisation: Send + Sync {
    async fn query(&self, request: &User) -> Result<(), ApplicationError>;
}

#[derive(Debug, Builder)]
pub struct PostAppClient {
    #[builder(setter(into))]
    pub query: AddUser,
}

#[async_trait]
impl PostAppInitialisation for PostAppClient {
    async fn query(&self, request: &User) -> Result<(), ApplicationError> {
        self.query.execute(request).await
    }
}