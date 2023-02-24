use cookie::Cookie;
use lambda_http::{run, service_fn, Error, IntoResponse, Request, RequestExt};
use oauth_flow::setup_tracing;
use oauth_flow::utils::api_helper::{ApiHelper, ApiResponse, HttpStatusCode};
use oauth_flow::utils::cookie::CookieExt;
use oauth_flow::utils::crypto::CriptoHelper;
use serde_json::json;
use std::collections::HashMap;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_tracing();

    run(service_fn(execute)).await
}

pub async fn execute(event: Request) -> Result<impl IntoResponse, Error> {
    println!("{event:?}");
    if let Some(client_id) = event.query_string_parameters().first("client_id") {
        // TODO move to the main
        let redirect_path = std::env::var("REDIRECT_PATH").expect("REDIRECT_PATH must be set");
        let oauth_authorize_uri =
            std::env::var("OAUTH_AUTHORIZE_URL").expect("OAUTH_AUTHORIZE_URL must be set");

        let host = event
            .headers()
            .get("Host")
            .expect("Cannot find host in the Request")
            .to_str()?;

        let state = Uuid::new_v4().to_string();
        let code_verifier = CriptoHelper::to_base64(CriptoHelper::random_bytes(128));

        let mut headers = HashMap::new();
        headers.insert(
            http::header::SET_COOKIE,
            Cookie::to_cookie_string(
                String::from("myApp"),
                HashMap::from([
                    (String::from("state"), state.to_owned()),
                    (String::from("code_verifier"), code_verifier.to_owned()),
                ]),
            ),
        );
        headers.insert(http::header::CONTENT_TYPE, "application/json".to_string());
        headers.insert(
            http::header::LOCATION,
            ApiHelper::build_url_from_hashmap(
                oauth_authorize_uri,
                HashMap::from([
                    ("client_id", client_id),
                    ("response_type", "code"),
                    ("state", &state),
                    (
                        "code_challenge",
                        &CriptoHelper::to_sha256_string(code_verifier),
                    ),
                    ("code_challenge_method", "S256"),
                    ("redirect_uri", &format!("https://{host}{redirect_path}")),
                ]),
            ),
        );

        return Ok(ApiHelper::response(ApiResponse {
            status_code: HttpStatusCode::Found,
            body: None,
            headers: Some(headers),
        }));
    }

    Ok(ApiHelper::response(ApiResponse {
        status_code: HttpStatusCode::NotFound,
        body: Some(json!({ "message": "Missing 'client_id' parameter in path" }).to_string()),
        headers: None,
    }))
}
