use aws_lambda_events::apigw::{
    ApiGatewayCustomAuthorizerRequestTypeRequest, ApiGatewayCustomAuthorizerResponse,
};
use chrono::Utc;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use website::{
    dtos::jwt::jwt_request::JwtRequest,
    setup_tracing,
    utils::injections::jwt::jwt_di::{JwtApiClient, JwtApiInitialisation},
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_tracing();

    let app_client = JwtApiClient::builder().build();

    run(service_fn(
        |event: LambdaEvent<ApiGatewayCustomAuthorizerRequestTypeRequest>| {
            handler(&app_client, event)
        },
    ))
    .await
}

pub async fn handler(
    app_client: &dyn JwtApiInitialisation,
    event: LambdaEvent<ApiGatewayCustomAuthorizerRequestTypeRequest>,
) -> Result<ApiGatewayCustomAuthorizerResponse, Error> {
    println!("EVENT {event:?}");

    let request = JwtRequest::validate(&event.payload);
    if let Some(request) = request {
        let audience = std::env::var("AUDIENCE").expect("AUDIENCE must be set");
        let token_issuer = std::env::var("TOKEN_ISSUER").expect("TOKEN_ISSUER must be set");

        let claims = app_client
            .validate_token(&request.authorization)
            .await
            .ok()
            .and_then(|claims| claims)
            .map(|claims| {
                claims.exp > Utc::now().timestamp()
                    && claims.aud == audience
                    && claims.iss == token_issuer
            });

        if claims.is_some() {
            return Ok(app_client.to_response("ALLOW".to_string(), "", request.method_arn));
        }
    }

    Ok(app_client.to_response("DENY".to_string(), "", event.payload.method_arn.unwrap()))
}
