use crate::{
    dtos::signup::singup_request::SignUpRequest,
    queries::user::add_user_query::{AddQuery, AddUser},
};
use async_trait::async_trait;
use typed_builder::TypedBuilder as Builder;

#[async_trait]
pub trait PostAppInitialisation: Send + Sync {
    async fn query(&self, request: &SignUpRequest) -> anyhow::Result<()>;
}

#[derive(Debug, Builder)]
pub struct PostAppClient {
    #[builder(setter(into))]
    pub query: AddUser,
}

#[async_trait]
impl PostAppInitialisation for PostAppClient {
    async fn query(&self, request: &SignUpRequest) -> anyhow::Result<()> {
        self.query.execute(request).await
    }
}
