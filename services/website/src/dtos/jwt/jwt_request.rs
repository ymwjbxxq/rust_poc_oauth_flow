use aws_lambda_events::apigw::ApiGatewayCustomAuthorizerRequestTypeRequest;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder as Builder;

#[derive(Debug, Builder, Serialize, Deserialize)]
pub struct JwtRequest {
    #[builder(default, setter(into))]
    pub method_arn: String,

    #[builder(default, setter(into))]
    pub authorization: String,
}

impl JwtRequest {
    pub fn validate(event: &ApiGatewayCustomAuthorizerRequestTypeRequest) -> Option<JwtRequest> {
        let method_arn = &event.method_arn;
        let authorization = event.headers.get("authorization");

        if let (Some(method_arn), Some(authorization)) = (method_arn, authorization) {
            return Some(
                Self::builder()
                    .method_arn(method_arn)
                    .authorization(authorization.to_str().unwrap())
                    .build(),
            );
        }

        println!("request not valid");
        None
    }
}
