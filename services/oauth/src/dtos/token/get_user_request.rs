use typed_builder::TypedBuilder as Builder;

#[derive(Debug, Builder)]
pub struct GetUserRequest {
    #[builder(default, setter(into))]
    pub user: String,

    #[builder(default, setter(into))]
    pub client_id: String,
}