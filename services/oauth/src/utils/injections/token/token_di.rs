use crate::{
    dtos::token::{get_key_request::GetKeyRequest, get_user_request::GetUserRequest},
    models::{csrf::CSRF, user::User},
    queries::{
        csrf::delete_csrf_query::{DeleteCSRF, DeleteCSRFQuery, DeleteCSRFRequest},
        csrf::get_csrf_query::{GetCSRF, GetCSRFQuery, GetCSRFRequest},
        token::get_key::{GetKey, GetKeyQuery},
        user::get_user_query::{GetUser, GetUserQuery},
    },
};
use async_trait::async_trait;
use typed_builder::TypedBuilder as Builder;

#[async_trait]
pub trait TokenAppInitialisation: Send + Sync {
    async fn get_csrf_query(&self, request: &GetCSRFRequest) -> anyhow::Result<Option<CSRF>>;

    async fn delete_csrf_query(&self, request: &DeleteCSRFRequest) -> anyhow::Result<()>;

    async fn get_user_query(&self, request: &GetUserRequest) -> anyhow::Result<Option<User>>;

    async fn get_key_query(&self, request: &GetKeyRequest) -> anyhow::Result<Option<String>>;
}

#[derive(Debug, Builder)]
pub struct TokenAppClient {
    #[builder(setter(into))]
    pub get_csrf_query: GetCSRF,

    #[builder(setter(into))]
    pub delete_csrf_query: DeleteCSRF,

    #[builder(setter(into))]
    pub get_user_query: GetUser,

    #[builder(setter(into))]
    pub get_key_query: GetKey,
}

#[async_trait]
impl TokenAppInitialisation for TokenAppClient {
    async fn get_csrf_query(&self, request: &GetCSRFRequest) -> anyhow::Result<Option<CSRF>> {
        self.get_csrf_query.execute(request).await
    }

    async fn delete_csrf_query(&self, request: &DeleteCSRFRequest) -> anyhow::Result<()> {
        self.delete_csrf_query.execute(request).await
    }

    async fn get_user_query(&self, request: &GetUserRequest) -> anyhow::Result<Option<User>> {
        self.get_user_query.execute(request).await
    }

    async fn get_key_query(&self, request: &GetKeyRequest) -> anyhow::Result<Option<String>> {
        self.get_key_query.execute(request).await
    }
}
