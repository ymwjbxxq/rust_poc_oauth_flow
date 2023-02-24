use lambda_http::{run, service_fn, Error, IntoResponse, Request, RequestExt};
use oauth_flow::setup_tracing;
use oauth_flow::utils::api_helper::{ApiHelper, ApiResponse, HttpStatusCode};
use oauth_flow::utils::cookie::CookieHelper;
use serde_json::json;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_tracing();

    run(service_fn(execute)).await
}

pub async fn execute(event: Request) -> Result<impl IntoResponse, Error> {
    println!("{event:?}");
    let cookie = CookieHelper::from_http_header(event.headers())?;
    let query_params = event.query_string_parameters();
    let state = query_params
        .first("state")
        .expect("state not found in query string");
    let cookie_state = cookie.get("state").expect("state not found in cookie");

    if state.eq(cookie_state) {
        let oauth_token_uri =
            std::env::var("OAUTH_TOKEN_URL").expect("OAUTH_TOKEN_URL must be set");
        let mut headers = HashMap::new();
        headers.insert(http::header::CONTENT_TYPE, "application/json".to_string());
        headers.insert(
            http::header::LOCATION,
            ApiHelper::build_url_from_hashmap(
                oauth_token_uri,
                HashMap::from([
                    (
                        "client_id",
                        query_params
                            .first("client_id")
                            .expect("client_id not found in query string"),
                    ),
                    ("grant_type", "authorization_code"),
                    (
                        "code",
                        query_params
                            .first("code")
                            .expect("code not found in query string"),
                    ),
                    (
                        "code_verifier",
                        cookie
                            .get("code_verifier")
                            .expect("code_verifier not found in query string"),
                    ),
                    (
                        "redirect_uri",
                        query_params
                            .first("redirect_uri")
                            .expect("redirect_uri not found in query string"),
                    ),
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
        status_code: HttpStatusCode::Unauthorized,
        body: Some(json!({ "message": "Unauthorized" }).to_string()),
        headers: None,
    }))
}
