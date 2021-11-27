use oauth_flow::utils::cookie::CookieHelper;
use lambda_http::{
  handler,
  lambda_runtime::{self, Context},
  IntoResponse, Request, RequestExt,
};
use log::LevelFilter;
use oauth_flow::utils::api_helper::{ApiHelper, ApiResponse, HttpStatusCode};
use serde_json::json;
use simple_logger::SimpleLogger;
use std::collections::HashMap;

type E = Box<dyn std::error::Error + Sync + Send + 'static>;

#[tokio::main]
async fn main() -> Result<(), E> {
  // Initialize service
  SimpleLogger::new()
    .with_level(LevelFilter::Info)
    .init()
    .unwrap();

  lambda_runtime::run(handler(execute)).await?;
  Ok(())
}

pub async fn execute(event: Request, _ctx: Context) -> Result<impl IntoResponse, E> {
  log::info!("EVENT {:?}", event);
  let cookie        = CookieHelper::from_http_header(event.headers())?;
  let query_params  = event.query_string_parameters();
  let state         = query_params.get("state").expect("state not found in query string");
  let cookie_state  = cookie.get("state").expect("state not found in cookie");

  if state.eq(cookie_state) {
    let oauth_token_uri = std::env::var("OAUTH_TOKEN_URL").expect("OAUTH_TOKEN_URL must be set");
    let mut headers = HashMap::new();
    headers.insert(http::header::CONTENT_TYPE, "application/json".to_string());
    headers.insert(
      http::header::LOCATION,
      ApiHelper::build_url_from_hashmap(
        oauth_token_uri,
        HashMap::from([
          ("client_id",     query_params.get("client_id").expect("client_id not found in query string")),
          ("grant_type",    "authorization_code"),
          ("code",          query_params.get("code").expect("code not found in query string")),
          ("code_verifier", cookie.get("code_verifier").expect("code_verifier not found in query string")),
          ("redirect_uri",  query_params.get("redirect_uri").expect("redirect_uri not found in query string")),
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
