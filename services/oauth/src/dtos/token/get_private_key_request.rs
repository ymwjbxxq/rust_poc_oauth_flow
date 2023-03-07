use typed_builder::TypedBuilder as Builder;

#[derive(Debug, Builder)]
pub struct GetPrivateKeyRequest {
    #[builder(setter(into))]
    pub client_id: String,

    #[builder(setter(into))]
    pub key_name: String,
}
