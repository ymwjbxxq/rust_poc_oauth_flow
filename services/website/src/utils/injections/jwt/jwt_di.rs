use async_trait::async_trait;
use aws_lambda_events::apigw::ApiGatewayCustomAuthorizerResponse;
use shared::{
    error::ApplicationError,
    utils::jwt::{Claims, Jwt},
};
use typed_builder::TypedBuilder as Builder;

#[async_trait]
pub trait JwtApiInitialisation: Send + Sync {
    async fn validate_token(&self, raw_token: &str, public_key: &str) -> Result<Option<Claims>, ApplicationError>;

    fn to_response(
        &self,
        effect: String,
        principal: &str,
        method_arn: String,
    ) -> ApiGatewayCustomAuthorizerResponse;
}

#[derive(Debug, Builder)]
pub struct JwtApiClient {}

#[async_trait]
impl JwtApiInitialisation for JwtApiClient {
    async fn validate_token(&self, raw_token: &str, public_key: &str) -> Result<Option<Claims>, ApplicationError> {
        Jwt::validate_token(raw_token, public_key).await
    }

    fn to_response(
        &self,
        effect: String,
        principal: &str,
        method_arn: String,
    ) -> ApiGatewayCustomAuthorizerResponse {
        Jwt::to_response(effect, principal, method_arn)
    }
}
