use crate::{
    models::{csrf::CSRF, user::User},
    queries::{
        csrf::delete_csrf_query::{DeleteCSRF, DeleteCSRFQuery, DeleteCSRFRequest},
        csrf::get_csrf_query::{GetCSRF, GetCSRFQuery, GetCSRFRequest},
        user::get_user_query::{GetUser, GetUserQuery},
    }, dtos::token::get_user_request::GetUserRequest,
};
use async_trait::async_trait;
use shared::error::ApplicationError;
use typed_builder::TypedBuilder as Builder;

#[async_trait]
pub trait TokenAppInitialisation: Send + Sync {
    async fn get_csrf_query(
        &self,
        request: &GetCSRFRequest,
    ) -> Result<Option<CSRF>, ApplicationError>;

    async fn delete_csrf_query(&self, request: &DeleteCSRFRequest) -> Result<(), ApplicationError>;

    async fn get_user_query(
        &self,
        request: &GetUserRequest,
    ) -> Result<Option<User>, ApplicationError>;
}

#[derive(Debug, Builder)]
pub struct TokenAppClient {
    #[builder(setter(into))]
    pub get_csrf_query: GetCSRF,

    #[builder(setter(into))]
    pub delete_csrf_query: DeleteCSRF,

    #[builder(setter(into))]
    pub get_user_query: GetUser,
}

#[async_trait]
impl TokenAppInitialisation for TokenAppClient {
    async fn get_csrf_query(
        &self,
        request: &GetCSRFRequest,
    ) -> Result<Option<CSRF>, ApplicationError> {
        self.get_csrf_query.execute(request).await
    }

    async fn delete_csrf_query(&self, request: &DeleteCSRFRequest) -> Result<(), ApplicationError> {
        self.delete_csrf_query.execute(request).await
    }

    async fn get_user_query(
        &self,
        request: &GetUserRequest,
    ) -> Result<Option<User>, ApplicationError> {
        self.get_user_query.execute(request).await
    }
}
