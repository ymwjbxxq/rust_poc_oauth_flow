use lambda_http::{run, service_fn, Error, IntoResponse, Request, RequestExt};
use oauth_flow::services::page::Page;
use oauth_flow::setup_tracing;
use oauth_flow::utils::api_helper::{ApiHelper, ApiResponse, HttpStatusCode};
use oauth_flow::utils::crypto::CriptoHelper;
use serde_json::json;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_tracing();

    let config = aws_config::load_from_env().await;
    let s3_client = aws_sdk_s3::Client::new(&config);

    run(service_fn(|event: Request| {
        execute(&s3_client, event)
    }))
    .await
}

pub async fn execute(
    s3_client: &aws_sdk_s3::Client,
    event: Request,
) -> Result<impl IntoResponse, Error> {
    println!("{event:?}");
    let bucket = std::env::var("CONFIG_BUCKETNAME").expect("CONFIG_BUCKETNAME must be set");

    let page = Page::builder()
        .s3_client(s3_client)
        .query_params(event.query_string_parameters())
        .bucket(bucket)
        .build();

    let optin_page = page
        .get_file_from_s3("optin")
        .await;

    if let Some(optin_page) = optin_page {
        let url = format!(
            "{}://{}{}",
            event.uri().scheme().unwrap(),
            event.uri().host().unwrap(),
            event.uri().path_and_query().unwrap()
        );
        let to_html = CriptoHelper::from_base64(optin_page)?.replace("#url#", &url);

        return Ok(ApiHelper::response(ApiResponse {
            status_code: HttpStatusCode::Success,
            body: Some(to_html),
            headers: Some(HashMap::from([(
                http::header::CONTENT_TYPE,
                "text/html".to_string(),
            )])),
        }));
    }

    Ok(ApiHelper::response(ApiResponse {
        status_code: HttpStatusCode::InternalServerError,
        body: Some(json!({ "message": "Cannot load UI" }).to_string()),
        headers: None,
    }))
}

