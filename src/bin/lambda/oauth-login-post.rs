use oauth_flow::utils::cookie::CookieExt;
use oauth_flow::utils::api_helper::BodyExt;
use cookie::Cookie;
use oauth_flow::models::user::User;
use oauth_flow::queries::get_user_query::GetUserQuery;
use oauth_flow::queries::get_user_query::LoginQuery;
use serde_json::json;
use std::collections::HashMap;
use oauth_flow::aws::client::AWSClient;
use oauth_flow::utils::api_helper::{ApiHelper, ApiResponse, HttpStatusCode};
use oauth_flow::aws::client::AWSConfig;
use lambda_http::{
  handler, RequestExt,
  lambda_runtime::{self, Context},
  IntoResponse, Request,
};
use log::LevelFilter;
use simple_logger::SimpleLogger;

type E = Box<dyn std::error::Error + Sync + Send + 'static>;

#[tokio::main]
async fn main() -> Result<(), E> {
  // Initialize service
  SimpleLogger::new()
    .with_level(LevelFilter::Info)
    .init()
    .unwrap();

  let config = aws_config::load_from_env().await;
  let aws_client = AWSConfig::set_config(config);
  let dynamo_db_client = aws_client.dynamo_client();
  let aws_client = AWSClient {
    dynamo_db_client: Some(dynamo_db_client),
    s3_client: None,
  };

  lambda_runtime::run(handler(|event: Request, ctx: Context| {
      execute(&aws_client, event, ctx)
  }))
  .await?;
  Ok(())
}

pub async fn execute(aws_client: &AWSClient, event: Request, _ctx: Context) -> Result<impl IntoResponse, E> {
  log::info!("EVENT {:?}", event);
  let result = login(aws_client, &event).await?;
  return Ok(match result {
    Some(user) => {
      let redirect_path = std::env::var("OAUTH_AUTHORIZE_PATH").expect("OAUTH_AUTHORIZE_PATH must be set");
      let query_params  = event.query_string_parameters();
      let host          = event.headers()
                                  .get("Host")
                                  .unwrap()
                                  .to_str()
                                  .unwrap();

      let mut headers = HashMap::new();
      headers.insert(http::header::SET_COOKIE, 
        Cookie::to_cookie_string(String::from("myOAuth"), HashMap::from([
          (String::from("email"), user.email.unwrap()),
          (String::from("is_consent"), user.is_consent.unwrap().to_string()),
          (String::from("is_optin"), user.is_optin.unwrap().to_string()),
        ])
      ));
      headers.insert(http::header::CONTENT_TYPE, "application/json".to_string());
      headers.insert(http::header::LOCATION, ApiHelper::build_url_from_querystring(format!("https://{}{}", host, redirect_path), query_params));

      ApiHelper::response(ApiResponse {
        status_code: HttpStatusCode::Found,
        body: None,
        headers: Some(headers),
      })
    }
    None => {
       ApiHelper::response(ApiResponse {
        status_code: HttpStatusCode::NotFound,
        body: Some(json!({ "message": "User not found" }).to_string()),
        headers: None,
      })
    }
  });
}

async fn login(aws_client: &AWSClient, event: &Request) -> Result<Option<User>, E> {
  let user          = event.get_from_body::<User>()?;
  let query_params  = event.query_string_parameters();
  let client_id     = query_params.get("client_id").expect("client_id not found");

  let user = LoginQuery::new(aws_client.dynamo_db_client.as_ref().unwrap())
      .execute(client_id, &user.unwrap().email.unwrap())
      .await?;

  Ok(user)
}