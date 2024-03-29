use crate::{
    dtos::login::login_user_request::LoginUserRequest,
    models::user::User,
    queries::{
        csrf::add_csrf_query::{AddCSRF, AddCSRFQuery, AddCSRFRequest},
        login::login_user_query::{LoginUser, LoginUserQuery},
    },
};
use async_trait::async_trait;
use typed_builder::TypedBuilder as Builder;

#[async_trait]
pub trait PostAppInitialisation: Send + Sync {
    async fn get_user_query(&self, request: &LoginUserRequest) -> anyhow::Result<Option<User>>;

    async fn add_csrf_query(&self, request: &AddCSRFRequest) -> anyhow::Result<()>;

    fn redirect_path(&self) -> &str;
}

#[derive(Debug, Builder)]
pub struct PostAppClient {
    #[builder(setter(into))]
    pub get_user_query: LoginUser,

    #[builder(setter(into))]
    pub add_csrf_query: AddCSRF,

    #[builder(setter(into))]
    pub redirect_path: String,
}

#[async_trait]
impl PostAppInitialisation for PostAppClient {
    async fn get_user_query(&self, request: &LoginUserRequest) -> anyhow::Result<Option<User>> {
        self.get_user_query.execute(request).await
    }

    async fn add_csrf_query(&self, request: &AddCSRFRequest) -> anyhow::Result<()> {
        self.add_csrf_query.execute(request).await
    }

    fn redirect_path(&self) -> &str {
        &self.redirect_path
    }
}
