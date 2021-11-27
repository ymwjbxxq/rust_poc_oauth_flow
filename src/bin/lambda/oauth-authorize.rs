use cookie::Cookie;
use lambda_http::{
  handler,
  lambda_runtime::{self, Context},
  IntoResponse, Request, RequestExt,
};
use log::LevelFilter;
use oauth_flow::utils::api_helper::{ApiHelper, ApiResponse, HttpStatusCode};
use oauth_flow::utils::cookie::CookieExt;
use oauth_flow::utils::cookie::CookieHelper;
use simple_logger::SimpleLogger;
use std::collections::HashMap;
use uuid::Uuid;

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
  let query_params  = event.query_string_parameters();
  let host          = event.headers().get("Host").unwrap().to_str().unwrap();

  let mut headers = HashMap::new();
  headers.insert(http::header::CONTENT_TYPE, "application/json".to_string());
  let cookie = CookieHelper::from_http_header(event.headers());
  if cookie.is_ok() {
    let mut cookie = cookie.unwrap();
    if cookie.get("is_optin").is_none() {
      let redirect_path =
        std::env::var("OAUTH_CUSTOM_OPTIN_PATH").expect("OAUTH_CUSTOM_OPTIN_PATH must be set");

      headers.insert(
        http::header::LOCATION,
        ApiHelper::build_url_from_querystring(
          format!("https://{}{}", host, redirect_path),
          query_params,
        ),
      );
      return Ok(ApiHelper::response(ApiResponse {
        status_code: HttpStatusCode::Found,
        body: None,
        headers: Some(headers),
      }));
    }

    if cookie.get("is_consent").is_none() {
      let redirect_path =
        std::env::var("OAUTH_CUSTOM_CONSENT_PATH").expect("OAUTH_CUSTOM_CONSENT_PATH must be set");

      headers.insert(
        http::header::LOCATION,
        ApiHelper::build_url_from_querystring(
          format!("https://{}{}", host, redirect_path),
          query_params,
        ),
      );
      return Ok(ApiHelper::response(ApiResponse {
        status_code: HttpStatusCode::Found,
        body: None,
        headers: Some(headers),
      }));
    }

    cookie.insert("code_challenge".to_owned(),  query_params.get("code_challenge").expect("code_challenge is not in query string").to_owned());
    headers.insert(http::header::SET_COOKIE,    Cookie::to_cookie_string(String::from("myOAuth"), cookie));
    headers.insert(http::header::LOCATION,      ApiHelper::build_url_from_hashmap(
                                                  query_params.get("redirect_uri").expect("redirect_uri is not in query string").to_owned(),
                                                  HashMap::from([
                                                    ("client_id",       query_params.get("client_id").expect("client_id is not in query string")),
                                                    ("code",            &Uuid::new_v4().to_string()),
                                                    ("state",           query_params.get("state").expect("state is not in query string")),
                                                    ("code_challenge",  query_params.get("code_challenge").expect("code_challenge is not in query string")),
                                                    ("redirect_uri",    query_params.get("redirect_uri").expect("redirect_uri is not in query string")),
                                                  ]),
                                                ));
  } else {
    log::info!("NO SESSION");
    let redirect_path = std::env::var("OAUTH_AUTHORIZE_LOGIN_PATH").expect("OAUTH_AUTHORIZE_LOGIN_PATH must be set");
    headers.insert(http::header::LOCATION,
      ApiHelper::build_url_from_querystring(
        format!("https://{}{}", host, redirect_path),
        query_params,
      ),
    );
  }

  Ok(ApiHelper::response(ApiResponse {
    status_code: HttpStatusCode::Found,
    body: None,
    headers: Some(headers),
  }))
}
