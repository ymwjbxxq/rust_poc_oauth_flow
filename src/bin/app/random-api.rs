use lambda_http::{run, service_fn, Error, IntoResponse, Request};
use oauth_flow::{setup_tracing, utils::api_helper::{ApiResponseType, ContentType, IsCors}};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_tracing();

    run(service_fn(execute)).await
}

pub async fn execute(event: Request) -> Result<impl IntoResponse, Error> {
    println!("{event:?}");

    Ok(ApiResponseType::Ok(
       json!({ "message": "Yeah" }).to_string(),
       ContentType::Json,
       IsCors::No,
    )
    .to_response())
}
