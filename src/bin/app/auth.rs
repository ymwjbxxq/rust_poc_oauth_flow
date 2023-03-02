use std::collections::HashMap;
use lambda_http::{run, service_fn, Error, IntoResponse, Request};
use oauth_flow::dtos::app::auth::auth_request::AuthRequest;
use oauth_flow::queries::get_csrf_query::{GetCSRF, GetCSRFRequest};
use oauth_flow::setup_tracing;
use oauth_flow::utils::api_helper::{ApiResponseType, IsCors};
use oauth_flow::utils::injections::app::auth::auth_di::{AuthAppClient, AuthAppInitialisation};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_tracing();

    let config = aws_config::load_from_env().await;
    let dynamodb_client = aws_sdk_dynamodb::Client::new(&config);

    let table_name = std::env::var("CSRF_TABLE_NAME").expect("CSRF_TABLE_NAME must be set");
    let get_csrf_query = GetCSRF::builder()
        .table_name(table_name)
        .client(dynamodb_client)
        .build();

    let oauth_token_uri = std::env::var("OAUTH_TOKEN_URL").expect("OAUTH_TOKEN_URL must be set");

    let app_client = AuthAppClient::builder()
        .get_csrf_query(get_csrf_query)
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
        let csrf_state = app_client
            .get_csrf_query(
                &GetCSRFRequest::builder()
                    .client_id(request.client_id.to_owned())
                    .sk(format!("state#{}", request.state.to_owned()))
                    .build(),
            )
            .await
            .ok()
            .and_then(|result| result);
        if csrf_state.is_some() {
            let target = ApiResponseType::build_url_from_hashmap(
                app_client.oauth_token_uri().to_owned(),
                HashMap::from([
                    ("client_id", request.client_id.as_ref()),
                    ("grant_type", "authorization_code"),
                    ("code", request.code.as_ref()),
                    ("code_verifier", request.code_challenge.as_ref()),
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
