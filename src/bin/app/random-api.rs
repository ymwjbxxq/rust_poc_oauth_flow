use lambda_http::{run, service_fn, Error, IntoResponse, Request};
use oauth_flow::{utils::api_helper::{ApiHelper, ApiResponse, HttpStatusCode}, setup_tracing};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_tracing();

    run(service_fn(execute)).await
}

pub async fn execute(event: Request) -> Result<impl IntoResponse, Error> {
    println!("{event:?}");

    Ok(ApiHelper::response(ApiResponse {
        status_code: HttpStatusCode::NotFound,
        body: Some(json!({ "message": "Yeah" }).to_string()),
        headers: None,
    }))
}
