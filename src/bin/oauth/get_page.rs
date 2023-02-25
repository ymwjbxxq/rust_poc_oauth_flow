use lambda_http::{run, service_fn, Error, IntoResponse, Request, RequestExt};
use oauth_flow::queries::get_page_query::{Page, PageRequest};
use oauth_flow::setup_tracing;
use oauth_flow::utils::api_helper::{ApiHelper, ApiResponse, HttpStatusCode};
use oauth_flow::utils::crypto::CriptoHelper;
use oauth_flow::utils::injections::oauth::get_page_di::{GetPageAppClient, GetPageAppInitialisation};
use serde_json::json;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_tracing();

    let config = aws_config::load_from_env().await;
    let s3_client = aws_sdk_s3::Client::new(&config);

    let bucket_name = std::env::var("CONFIG_BUCKETNAME").expect("CONFIG_BUCKETNAME must be set");
    let page_name = std::env::var("PAGE_NAME").expect("PAGE_NAME must be set");
    let query = Page::builder()
        .bucket_name(bucket_name)
        .page_name(page_name)
        .client(s3_client)
        .build();

    let app_client = GetPageAppClient::builder().query(query).build();

    run(service_fn(|event: Request| execute(&app_client, event))).await
}

pub async fn execute(
    app_client: &dyn GetPageAppInitialisation,
    event: Request,
) -> Result<impl IntoResponse, Error> {
    println!("{event:?}");

    let query_param = event.query_string_parameters();
    let client_id = query_param
        .first("client_id")
        .expect("client_id not found in query string");

    let consent_page = app_client
        .query(
            &PageRequest::builder()
                .client_id(client_id)
                .build(),
        )
        .await
        .ok()
        .and_then(|page| page);

    if let Some(consent_page) = consent_page {
        let url = format!(
            "{}://{}{}",
            event.uri().scheme().unwrap(),
            event.uri().host().unwrap(),
            event.uri().path_and_query().unwrap()
        );
        let to_html = CriptoHelper::from_base64(consent_page)?.replace("#url#", &url);

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
