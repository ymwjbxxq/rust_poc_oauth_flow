use crate::{
    dtos::signup::singup_request::SignUpRequest, queries::user::add_user_query::{AddUser, AddQuery}
};
use async_trait::async_trait;
use shared::error::ApplicationError;
use typed_builder::TypedBuilder as Builder;

#[async_trait]
pub trait PostAppInitialisation: Send + Sync {
    async fn query(&self, request: &SignUpRequest) -> Result<(), ApplicationError>;
}

#[derive(Debug, Builder)]
pub struct PostAppClient {
    #[builder(setter(into))]
    pub query: AddUser,
}

#[async_trait]
impl PostAppInitialisation for PostAppClient {
    async fn query(&self, request: &SignUpRequest) -> Result<(), ApplicationError> {
        self.query.execute(request).await
    }
}
