use aws_lambda_events::apigw::{
    ApiGatewayCustomAuthorizerRequestTypeRequest, ApiGatewayCustomAuthorizerResponse,
};
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use oauth_flow::{
    dtos::app::authorizer::authorizer_request::AuthorizerRequest,
    setup_tracing,
    utils::{injections::app::authorizer::authorizer_di::{AuthorizerAppInitialisation, AuthorizerAppClient}, jwt::Jwt},
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_tracing();

    let jwt = Jwt::new("privateKey");
    let app_client = AuthorizerAppClient::builder().jwt(jwt).build();

    run(service_fn(
        |event: LambdaEvent<ApiGatewayCustomAuthorizerRequestTypeRequest>| {
            handler(&app_client, event)
        },
    ))
    .await
}

pub async fn handler(
    app_client: &dyn AuthorizerAppInitialisation,
    event: LambdaEvent<ApiGatewayCustomAuthorizerRequestTypeRequest>,
) -> Result<ApiGatewayCustomAuthorizerResponse, Error> {
    println!("EVENT {event:?}");

    let request = AuthorizerRequest::validate(&event.payload);
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
