use lambda_http::{run, service_fn, Error, IntoResponse, Request};
use serde_json::json;
use shared::utils::api_helper::{ApiResponseType, ContentType, IsCors};
use website::setup_tracing;

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_tracing();

    run(service_fn(handler)).await
}

pub async fn handler(event: Request) -> anyhow::Result<impl IntoResponse> {
    println!("{event:?}");

    Ok(ApiResponseType::Ok(
        json!({ "message": "Yeah" }).to_string(),
        ContentType::Json,
        IsCors::Yes,
    )
    .to_response())
}
