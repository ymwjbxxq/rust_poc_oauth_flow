use lambda_http::{
  handler,
  lambda_runtime::{self, Context},
  IntoResponse, Request,
};
use log::LevelFilter;
use oauth_flow::utils::api_helper::{ApiHelper, ApiResponse, HttpStatusCode};
use serde_json::json;
use simple_logger::SimpleLogger;
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

  Ok(ApiHelper::response(ApiResponse {
    status_code: HttpStatusCode::NotFound,
    body: Some(json!({ "message": "Yeah" }).to_string()),
    headers: None,
  }))
}