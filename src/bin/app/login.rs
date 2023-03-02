use chrono::Utc;
use lambda_http::{run, service_fn, Error, IntoResponse, Request};
use oauth_flow::dtos::app::login::login_request::LoginRequest;
use oauth_flow::queries::add_csrf_query::{AddCSRF, AddCSRFRequest};
use oauth_flow::setup_tracing;
use oauth_flow::utils::api_helper::{ApiResponseType, ContentType, IsCors};
use oauth_flow::utils::crypto::CriptoHelper;
use oauth_flow::utils::injections::app::login::login_di::{LoginAppClient, LoginAppInitialisation};
use serde_json::json;
use std::collections::HashMap;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_tracing();

    let config = aws_config::load_from_env().await;
    let dynamodb_client = aws_sdk_dynamodb::Client::new(&config);

    let redirect_path = std::env::var("REDIRECT_PATH").expect("REDIRECT_PATH must be set");
    let oauth_authorize_uri =
        std::env::var("OAUTH_AUTHORIZE_URL").expect("OAUTH_AUTHORIZE_URL must be set");
    let table_name = std::env::var("CSRF_TABLE_NAME").expect("CSRF_TABLE_NAME must be set");

    let add_csrf_query = AddCSRF::builder()
        .table_name(table_name)
        .client(dynamodb_client)
        .build();

    let app_client = LoginAppClient::builder()
        .redirect_path(redirect_path)
        .oauth_authorize_uri(oauth_authorize_uri)
        .add_csrf_query(add_csrf_query)
        .build();

    run(service_fn(|event: Request| handler(&app_client, event))).await
}

pub async fn handler(
    app_client: &dyn LoginAppInitialisation,
    event: Request,
) -> Result<impl IntoResponse, Error> {
    println!("{event:?}");

    let request = LoginRequest::validate(&event);
    if let Some(request) = request {
        let state = Uuid::new_v5(
            &Uuid::NAMESPACE_OID,
            format!("{}{}", Uuid::new_v4(), Utc::now().timestamp_millis()).as_bytes(),
        )
        .to_string();

        let result = app_client
            .add_csrf_query(
                &AddCSRFRequest::builder()
                    .client_id(request.client_id.to_owned())
                    .sk(format!("state#{}", state.to_owned()))
                    .data(None)
                    .build(),
            )
            .await;
        if result.is_ok() {
            let target = ApiResponseType::build_url_from_hashmap(
                app_client.oauth_authorize_uri().to_owned(),
                HashMap::from([
                    ("client_id", request.client_id.as_ref()),
                    ("response_type", "code"),
                    ("state", &state),
                    (
                        "code_challenge",
                        &CriptoHelper::to_sha256_string(CriptoHelper::to_base64(
                            CriptoHelper::random_bytes(128),
                        )),
                    ),
                    ("code_challenge_method", "S256"),
                    (
                        "redirect_uri",
                        &format!("https://{}{}", request.host, app_client.redirect_path()),
                    ),
                ]),
            );
            return Ok(ApiResponseType::Ok(
                json!({ "url": target }).to_string(),
                ContentType::Json,
                IsCors::Yes,
            )
            .to_response());
        }
    }

    Ok(ApiResponseType::Conflict(
        json!({ "errors": ["Request is missing parameters"] }).to_string(),
        IsCors::Yes,
    )
    .to_response())
}
