use chrono::{Duration, Utc};
use lambda_http::{run, service_fn, Error, IntoResponse, Request};
use oauth_flow::dtos::oauth::token::toekn_request::TokenRequest;
use oauth_flow::utils::api_helper::{ContentType, IsCors};
use oauth_flow::utils::crypto::CriptoHelper;
use oauth_flow::utils::jwt::{Claims, Jwt};
use oauth_flow::{setup_tracing, utils::api_helper::ApiResponseType};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_tracing();

    run(service_fn(|event: Request| handler(event))).await
}

pub async fn handler(event: Request) -> Result<impl IntoResponse, Error> {
    println!("{event:?}");

    let request = TokenRequest::validate(&event);
    if let Some(request) = request {
        let base64_digest = CriptoHelper::to_sha256_string(request.code_verifier);

        if request.code_challenge.eq(&base64_digest) {
            let claims = Claims::builder()
                .sub(request.user)
                .company(request.client_id)
                .exp((Utc::now() + Duration::minutes(60)).timestamp())
                .build();

            let token = Jwt::new("private_key")
                .encode(&claims)
                .ok()
                .and_then(|token| token);

            if let Some(token) = token {
                return Ok(ApiResponseType::Ok(
                    json!({ "token": token }).to_string(),
                    ContentType::Json,
                    IsCors::No,
                )
                .to_response());
            }
        }
    }

    println!("token Unauthorized");
    Ok(ApiResponseType::Forbidden(
        json!({ "errors": ["Unauthorized"] }).to_string(),
        IsCors::No,
    )
    .to_response())
}
