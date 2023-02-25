use crate::{
    error::ApplicationError,
    models::user::User,
    queries::get_user_query::{GetUser, GetUserQuery}, dtos::login::get_user_request::GetUserRequest,
};
use async_trait::async_trait;
use typed_builder::TypedBuilder as Builder;

#[async_trait]
pub trait PostAppInitialisation: Send + Sync {
    async fn query(&self, request: &GetUserRequest) -> Result<Option<User>, ApplicationError>;
    fn redirect_path(&self) -> &str;
}

#[derive(Debug, Builder)]
pub struct PostAppClient {
    #[builder(setter(into))]
    pub query: GetUser,

    #[builder(setter(into))]
    pub redirect_path: String,
}

#[async_trait]
impl PostAppInitialisation for PostAppClient {
    async fn query(&self, request: &GetUserRequest) -> Result<Option<User>, ApplicationError> {
        self.query.execute(request).await
    }

    fn redirect_path(&self) -> &str {
        &self.redirect_path
    }
}
