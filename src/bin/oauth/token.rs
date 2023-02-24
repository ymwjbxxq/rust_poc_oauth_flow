use jsonwebtoken::{encode, EncodingKey, Header};
use lambda_http::{run, service_fn, Error, IntoResponse, Request, RequestExt};
use oauth_flow::setup_tracing;
use oauth_flow::utils::api_helper::{ApiHelper, ApiResponse, HttpStatusCode};
use oauth_flow::utils::cookie::CookieHelper;
use oauth_flow::utils::crypto::CriptoHelper;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_tracing();

    run(service_fn(|event: Request| {
        execute(event)
    }))
    .await
}

pub async fn execute(event: Request) -> Result<impl IntoResponse, Error> {
    println!("{event:?}");

    let cookie = CookieHelper::from_http_header(event.headers())?;
    let query_params = event.query_string_parameters();
    let code_verifier = query_params
        .first("code_verifier")
        .expect("code_verifier not found in query string");
    let code_challenge = cookie
        .get("code_challenge")
        .expect("code_challenge not found in cookie");
    let base64_digest = CriptoHelper::to_sha256_string(code_verifier);

    if code_challenge.eq(&base64_digest) {
        let my_claims = Claims {
            sub: "b@b.com".to_owned(),
            company: "ACME".to_owned(),
            exp: 10000000000,
        };
        let token = encode(
            &Header::default(),
            &my_claims,
            &EncodingKey::from_secret("privateKey".as_bytes()),
        )?;

        return Ok(ApiHelper::response(ApiResponse {
            status_code: HttpStatusCode::Success,
            body: Some(json!({ "message": token }).to_string()),
            headers: None,
        }));
    }

    println!("token Unauthorized");
    Ok(ApiHelper::response(ApiResponse {
        status_code: HttpStatusCode::Unauthorized,
        body: Some(json!({ "message": "Unauthorized" }).to_string()),
        headers: None,
    }))
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}
