use aws_lambda_events::apigw::ApiGatewayCustomAuthorizerRequestTypeRequest;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder as Builder;

#[derive(Debug, Builder, Serialize, Deserialize)]
pub struct AuthorizerRequest {
    #[builder(default, setter(into))]
    pub method_arn: String,

    #[builder(default, setter(into))]
    pub authorization: String,
}

impl AuthorizerRequest {
    pub fn validate(event: &ApiGatewayCustomAuthorizerRequestTypeRequest) -> Option<AuthorizerRequest> {
        let method_arn = &event.method_arn;
        let authorization = event.headers.get("Authorization");

        if let (Some(method_arn), Some(authorization)) = (method_arn, authorization) {
            return Some(
                Self::builder()
                    .method_arn(method_arn)
                    .authorization(authorization.to_str().unwrap())
                    .build(),
            );
        }
        None
    }
}
