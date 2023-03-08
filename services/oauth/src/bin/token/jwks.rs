use lambda_http::{run, service_fn, Error, IntoResponse, Request};
use oauth::dtos::token::get_key_request::GetKeyRequest;
use oauth::dtos::token::jwks_request::JwksRequest;
use oauth::queries::token::get_key::GetKey;
use oauth::setup_tracing;
use oauth::utils::injections::token::jwks_di::{JwksAppClient, JwksAppInitialisation};
use serde_json::json;
use shared::utils::api_helper::{ApiResponseType, ContentType, IsCors};

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_tracing();

    let config = aws_config::load_from_env().await;

    let get_key_query = GetKey::builder()
        .client(aws_sdk_ssm::Client::new(&config))
        .build();

    let app_client = JwksAppClient::builder()
        .get_key_query(get_key_query)
        .build();

    run(service_fn(|event: Request| handler(&app_client, event))).await
}

pub async fn handler<'a>(
    app_client: &dyn JwksAppInitialisation,
    event: Request,
) -> Result<impl IntoResponse, Error> {
    println!("{event:?}");

    let mut messages: Vec<String> = Vec::new();
    let request = JwksRequest::validate(&event);

    if let Some(request) = request {
        let public_key = app_client
            .get_key_query(
                &GetKeyRequest::builder()
                    .client_id(request.client_id.to_lowercase())
                    .key_name(format!("/{}/public_key", request.client_id.to_lowercase()))
                    .build(),
            )
            .await
            .ok()
            .and_then(|result| result);
        if let Some(public_key) = public_key {
            return Ok(ApiResponseType::Ok(
                json!({ "public_key": public_key }).to_string(),
                ContentType::Json,
                IsCors::Yes,
            )
            .to_response());
        } else {
            messages.push("problem retrieving the jwks.json".to_string());
        }
    } else {
        messages.push("Invalid request".to_string());
    }

    Ok(
        ApiResponseType::Forbidden(json!({ "errors": messages }).to_string(), IsCors::Yes)
            .to_response(),
    )
}
