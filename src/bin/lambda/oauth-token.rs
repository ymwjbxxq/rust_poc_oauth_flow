use oauth_flow::utils::cookie::CookieHelper;
use oauth_flow::utils::api_helper::{ApiHelper, ApiResponse, HttpStatusCode};
use oauth_flow::utils::crypto::CriptoHelper;
use lambda_http::{
  handler,
  lambda_runtime::{self, Context},
  IntoResponse, Request, RequestExt,
};
use log::LevelFilter;
use serde_json::json;
use simple_logger::SimpleLogger;
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, Header, EncodingKey};

type E = Box<dyn std::error::Error + Sync + Send + 'static>;

#[tokio::main]
async fn main() -> Result<(), E> {
  // Initialize service
  SimpleLogger::new()
    .with_level(LevelFilter::Info)
    .init()
    .unwrap();

  lambda_runtime::run(handler(|event: Request, ctx: Context| {
      execute(event, ctx)
  }))
  .await?;
  Ok(())
}

pub async fn execute(event: Request, _ctx: Context) -> Result<impl IntoResponse, E> {
  log::info!("EVENT {:?}", event);

  let cookie          = CookieHelper::from_http_header(event.headers())?;
  let query_params    = event.query_string_parameters();
  let code_verifier   = query_params.get("code_verifier").expect("code_verifier not found in query string");
  let code_challenge  = cookie.get("code_challenge").expect("code_challenge not found in cookie");
  let base64_digest   = CriptoHelper::to_sha256_string(&code_verifier);

  if code_challenge.eq(&base64_digest) {
    let my_claims = Claims { 
      sub: "b@b.com".to_owned(), 
      company: "ACME".to_owned(), 
      exp: 10000000000
    };
    let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret("privateKey".as_bytes()))?;

    return Ok(ApiHelper::response(ApiResponse {
      status_code: HttpStatusCode::Success,
      body: Some(json!({ "message": token }).to_string()),
      headers: None,
    }));
  }

  log::info!("token Unauthorized");
  return Ok(ApiHelper::response(ApiResponse {
    status_code: HttpStatusCode::Unauthorized,
    body: Some(json!({ "message": "Unauthorized" }).to_string()),
    headers: None,
  }));
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
  sub: String,
  company: String,
  exp: usize,
}