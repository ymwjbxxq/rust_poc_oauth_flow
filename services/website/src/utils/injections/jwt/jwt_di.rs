use async_trait::async_trait;
use aws_lambda_events::apigw::ApiGatewayCustomAuthorizerResponse;
use typed_builder::TypedBuilder as Builder;

use crate::{
    error::ApplicationError,
    utils::jwt::{Claims, Jwt},
};

#[async_trait]
pub trait JwtApiInitialisation: Send + Sync {
    async fn validate_token(&self, raw_token: &str) -> Result<Option<Claims>, ApplicationError>;
    
    fn to_response(
        &self,
        effect: String,
        principal: &str,
        method_arn: String,
    ) -> ApiGatewayCustomAuthorizerResponse;
}

#[derive(Debug, Builder)]
pub struct JwtApiClient {
    #[builder(setter(into))]
    pub jwt: Jwt,
}

#[async_trait]
impl JwtApiInitialisation for JwtApiClient {
    async fn validate_token(&self, raw_token: &str) -> Result<Option<Claims>, ApplicationError> {
        self.jwt.validate_token(raw_token).await
    }

    fn to_response(
        &self,
        effect: String,
        principal: &str,
        method_arn: String,
    ) -> ApiGatewayCustomAuthorizerResponse {
        self.jwt.to_response(effect, principal, method_arn)
    }
}
