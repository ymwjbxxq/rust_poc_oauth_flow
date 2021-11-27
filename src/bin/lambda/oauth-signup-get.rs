use oauth_flow::utils::crypto::{CriptoHelper};
use oauth_flow::aws::client::AWSClient;
use oauth_flow::utils::api_helper::{ApiHelper, ApiResponse, HttpStatusCode};
use oauth_flow::aws::client::AWSConfig;
use lambda_http::{
  handler, RequestExt,
  lambda_runtime::{self, Context},
  IntoResponse, Request,
};
use log::LevelFilter;
use serde_json::json;
use simple_logger::SimpleLogger;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
  let s3_client = aws_client.s3_client();
  let aws_client = AWSClient {
    dynamo_db_client: None,
    s3_client: Some(s3_client),
  };

  lambda_runtime::run(handler(|event: Request, ctx: Context| {
      execute(&aws_client, event, ctx)
  }))
  .await?;
  Ok(())
}

pub async fn execute(aws_client: &AWSClient, event: Request, _ctx: Context) -> Result<impl IntoResponse, E> {
  log::info!("EVENT {:?}", event);
  let result = load_ui(aws_client, &event).await?;
  return Ok(match result {
    Some(result) => {
      let url = format!("{}://{}{}", 
                          event.uri().scheme().unwrap(),
                          event.uri().host().unwrap(),
                          event.uri().path_and_query().unwrap());
      let to_html = CriptoHelper::from_base64(result)?
                                  .replace("#url#", &url);

      ApiHelper::response(ApiResponse {
        status_code: HttpStatusCode::Success,
        body: Some(to_html),
        headers: Some(HashMap::from([(http::header::CONTENT_TYPE, "text/html".to_string())])),
      })
    }
    None => {
        ApiHelper::response(ApiResponse {
          status_code: HttpStatusCode::InternalServerError,
          body: Some(json!({ "message": "Cannot load UI" }).to_string()),
          headers: None,
        })
    }
  });
}

async fn load_ui(aws_client: &AWSClient, event: &Request) -> Result<Option<String>, E> {
  let query_params  = event.query_string_parameters();
  let client_id     = query_params.get("client_id").expect("client_id not found");
  let file          = format!("{}.json", client_id);
  let bucket        = std::env::var("CONFIG_BUCKETNAME").expect("CONFIG_BUCKETNAME must be set");

  let result = aws_client.s3_client.as_ref().unwrap()
        .get_object()
        .bucket(bucket)
        .key(file)
        .response_content_type("application/json")
        .send()
        .await?;

  let bytes     = result.body.collect().await?.into_bytes();
  let response  = std::str::from_utf8(&bytes)?;
  let ui: UI    = serde_json::from_str(response).unwrap();

  Ok(Some(ui.signup))
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UI {
  pub signup: String,
}
