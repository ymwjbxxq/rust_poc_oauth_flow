use lambda_http::{run, service_fn, Error, IntoResponse, Request};
use oauth_flow::dtos::app::auth::auth_request::AuthRequest;
use oauth_flow::setup_tracing;
use oauth_flow::utils::api_helper::{ApiResponseType, IsCors};
use oauth_flow::utils::injections::app::auth::auth_di::{AuthAppClient, AuthAppInitialisation};
use serde_json::json;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_tracing();

    let oauth_token_uri = std::env::var("OAUTH_TOKEN_URL").expect("OAUTH_TOKEN_URL must be set");

    let app_client = AuthAppClient::builder()
        .oauth_token_uri(oauth_token_uri)
        .build();

    run(service_fn(|event: Request| handler(&app_client, event))).await
}

pub async fn handler(
    app_client: &dyn AuthAppInitialisation,
    event: Request,
) -> Result<impl IntoResponse, Error> {
    println!("{event:?}");

    let request = AuthRequest::validate(&event);
    if let Some(request) = request {
        if request.querystring_state == request.cookie_state {
            let target = ApiResponseType::build_url_from_hashmap(
                app_client.oauth_token_uri().to_owned(),
                HashMap::from([
                    ("client_id", request.client_id.as_ref()),
                    ("grant_type", "authorization_code"),
                    ("code", request.code.as_ref()),
                    ("code_verifier", request.code_verifier.as_ref()),
                    ("redirect_uri", request.redirect_uri.as_ref()),
                ]),
            );
            return Ok(ApiResponseType::Found(target, IsCors::Yes).to_response());
        }
    }

    Ok(ApiResponseType::Forbidden(
        json!({ "errors": ["Unauthorized"] }).to_string(),
        IsCors::Yes,
    )
    .to_response())
}
