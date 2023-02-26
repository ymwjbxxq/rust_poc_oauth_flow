use aws_lambda_events::apigw::{
    ApiGatewayCustomAuthorizerRequestTypeRequest, ApiGatewayCustomAuthorizerResponse,
};
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use oauth_flow::{
    dtos::app::jwt::jwt_request::JwtRequest,
    setup_tracing,
    utils::{injections::app::jwt::jwt_di::{JwtApiInitialisation, JwtApiClient}, jwt::Jwt},
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_tracing();

    let jwt = Jwt::new("privateKey");
    let app_client = JwtApiClient::builder().jwt(jwt).build();

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
        let claims = app_client
            .validate_token(&request.authorization)
            .await
            .ok()
            .and_then(|claims| claims);

        if claims.is_some() {
            return Ok(app_client.to_response("ALLOW".to_string(), "", request.method_arn));
        }
    }

    Ok(app_client.to_response("DENY".to_string(), "", event.payload.method_arn.unwrap()))
}
