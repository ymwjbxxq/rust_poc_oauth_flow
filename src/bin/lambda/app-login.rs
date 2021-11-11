use oauth_flow::utils::crypto::CriptoHelper;
use oauth_flow::utils::cookie::CookieExt;
use cookie::Cookie;
use lambda_http::{
  handler,
  RequestExt,
  lambda_runtime::{self, Context},
  IntoResponse, Request,
};
use std::collections::HashMap;
use log::LevelFilter;
use simple_logger::SimpleLogger;
use oauth_flow::utils::api_helper::{ApiHelper, ApiResponse, HttpStatusCode};
use serde_json::json;
use uuid::Uuid;

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
  let query_params = event.query_string_parameters();
  let client_id = match query_params.get("client_id") {
    Some(client_id) => client_id,
    None => {
      return Ok(ApiHelper::response(ApiResponse {
        status_code: HttpStatusCode::NotFound,
        body: Some(json!({ "message": "Missing 'client_id' parameter in path" }).to_string()),
        headers: None,
      }));
    }
  };

  let redirect_path       = std::env::var("REDIRECT_PATH").expect("REDIRECT_PATH must be set");
  let host                = event.headers().get("Host").unwrap().to_str().unwrap();
  let redirect_uri        = format!("https://{}/{}", host, redirect_path);
  let oauth_authorize_uri = std::env::var("OAUTH_AUTHORIZE_URL").expect("OAUTH_AUTHORIZE_URL must be set");

  let state = Uuid::new_v4().to_string();
  let code_verifier = CriptoHelper::to_base64(CriptoHelper::random_bytes(128));

  let mut headers = HashMap::new();
  headers.insert(http::header::SET_COOKIE, 
    Cookie::to_cookie_string(String::from("myApp"), HashMap::from([
      (String::from("state"), state.to_owned()),
      (String::from("code_verifier"), code_verifier.to_owned()),
    ])
  ));
  headers.insert(http::header::CONTENT_TYPE, "application/json".to_string());
  headers.insert(http::header::LOCATION,
                  ApiHelper::build_url_from_hashmap(
                    oauth_authorize_uri,
                    HashMap::from([
                      ("client_id", client_id),
                      ("response_type", "code"),
                      ("state", &state),
                      ("code_challenge", &CriptoHelper::to_sha256_string(code_verifier.to_owned())),
                      ("code_challenge_method", "S256"),
                      ("redirect_uri", &redirect_uri),
                    ]),
                  ),
  );

  return Ok(ApiHelper::response(ApiResponse {
        status_code: HttpStatusCode::Found,
        body: None,
        headers: Some(headers),
      }));
}