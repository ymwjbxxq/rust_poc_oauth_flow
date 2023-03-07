use crate::dtos::token::get_private_key_request::GetPrivateKeyRequest;
use async_trait::async_trait;
use aws_sdk_ssm::Client;
use shared::error::ApplicationError;
use typed_builder::TypedBuilder as Builder;

#[derive(Debug, Builder)]
pub struct GetPrivateKey {
    #[builder(setter(into))]
    client: Client,
}

#[async_trait]
pub trait GetPrivateKeyQuery {
    async fn execute(
        &self,
        request: &GetPrivateKeyRequest,
    ) -> Result<Option<String>, ApplicationError>;
}

#[async_trait]
impl GetPrivateKeyQuery for GetPrivateKey {
    async fn execute(
        &self,
        request: &GetPrivateKeyRequest,
    ) -> Result<Option<String>, ApplicationError> {
        let res = self
            .client
            .get_parameter()
            .name(request.key_name.to_owned())
            .with_decryption(true)
            .send()
            .await?;

        Ok(res.parameter.and_then(|x| x.value))
    }
}
