use typed_builder::TypedBuilder as Builder;

#[derive(Debug, Builder)]
pub struct GetKeyRequest {
    #[builder(setter(into))]
    pub client_id: String,

    #[builder(setter(into))]
    pub key_name: String,
}
