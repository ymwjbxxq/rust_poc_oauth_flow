use cookie::Cookie;
use http::{HeaderMap, HeaderValue};
use lambda_http::{run, service_fn, Error, IntoResponse, Request};
use oauth_flow::dtos::app::login::login_request::LoginRequest;
use oauth_flow::setup_tracing;
use oauth_flow::utils::api_helper::{ApiResponseType, IsCors};
use oauth_flow::utils::cookie::CookieExt;
use oauth_flow::utils::crypto::CriptoHelper;
use oauth_flow::utils::injections::app::login::login_di::{LoginAppClient, LoginAppInitialisation};
use serde_json::json;
use std::collections::HashMap;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_tracing();
    let redirect_path = std::env::var("REDIRECT_PATH").expect("REDIRECT_PATH must be set");
    let oauth_authorize_uri =
        std::env::var("OAUTH_AUTHORIZE_URL").expect("OAUTH_AUTHORIZE_URL must be set");

    let app_client = LoginAppClient::builder()
        .redirect_path(redirect_path)
        .oauth_authorize_uri(oauth_authorize_uri)
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
        let state = Uuid::new_v4().to_string();
        let code_verifier = CriptoHelper::to_base64(CriptoHelper::random_bytes(128));

        let cookie = Cookie::to_cookie_string(
            String::from("myApp"),
            HashMap::from([
                (String::from("state"), state.to_owned()),
                (String::from("code_verifier"), code_verifier.to_owned()),
            ]),
        );
        let mut headers = HeaderMap::new();
        headers.insert(http::header::SET_COOKIE, HeaderValue::from_str(&cookie)?);
        let target = ApiResponseType::build_url_from_hashmap(
            app_client.oauth_authorize_uri().to_owned(),
            HashMap::from([
                ("client_id", request.client_id.as_ref()),
                ("response_type", "code"),
                ("state", &state),
                (
                    "code_challenge",
                    &CriptoHelper::to_sha256_string(code_verifier),
                ),
                ("code_challenge_method", "S256"),
                (
                    "redirect_uri",
                    &format!("https://{}{}", request.host, app_client.redirect_path()),
                ),
            ]),
        );

        return Ok(
            ApiResponseType::FoundWithCustomHeaders(target, IsCors::Yes, headers).to_response(),
        );
    }

    Ok(ApiResponseType::Conflict(
        json!({ "errors": ["Request is missing parameters"] }).to_string(),
        IsCors::Yes,
    )
    .to_response())
}
