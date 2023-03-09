use async_trait::async_trait;
use aws_lambda_events::apigw::ApiGatewayCustomAuthorizerResponse;
use shared::utils::jwt::{Claims, Jwt};
use typed_builder::TypedBuilder as Builder;

#[async_trait]
pub trait JwtApiInitialisation: Send + Sync {
    async fn validate_token(&self, raw_token: &str) -> anyhow::Result<Option<Claims>>;

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
    async fn validate_token(&self, raw_token: &str) -> anyhow::Result<Option<Claims>> {
        Jwt::validate_token(raw_token).await
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
