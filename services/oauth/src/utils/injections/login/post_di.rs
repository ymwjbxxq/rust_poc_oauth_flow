use crate::{
    dtos::login::get_user_request::GetUserRequest,
    models::user::User,
    queries::{
        add_csrf_query::{AddCSRF, AddCSRFQuery, AddCSRFRequest},
        get_user_query::{GetUser, GetUserQuery},
    },
};
use async_trait::async_trait;
use shared::error::ApplicationError;
use typed_builder::TypedBuilder as Builder;

#[async_trait]
pub trait PostAppInitialisation: Send + Sync {
    async fn get_user_query(
        &self,
        request: &GetUserRequest,
    ) -> Result<Option<User>, ApplicationError>;

    async fn add_csrf_query(&self, request: &AddCSRFRequest) -> Result<(), ApplicationError>;

    fn redirect_path(&self) -> &str;
}

#[derive(Debug, Builder)]
pub struct PostAppClient {
    #[builder(setter(into))]
    pub get_user_query: GetUser,

    #[builder(setter(into))]
    pub add_csrf_query: AddCSRF,

    #[builder(setter(into))]
    pub redirect_path: String,
}

#[async_trait]
impl PostAppInitialisation for PostAppClient {
    async fn get_user_query(
        &self,
        request: &GetUserRequest,
    ) -> Result<Option<User>, ApplicationError> {
        self.get_user_query.execute(request).await
    }

    async fn add_csrf_query(&self, request: &AddCSRFRequest) -> Result<(), ApplicationError> {
        self.add_csrf_query.execute(request).await
    }

    fn redirect_path(&self) -> &str {
        &self.redirect_path
    }
}
