use lambda_http::{run, service_fn, Error, IntoResponse, Request};
use oauth_flow::dtos::oauth::load_page::page_request::PageRequest;
use oauth_flow::queries::get_page_query::Page;
use oauth_flow::setup_tracing;
use oauth_flow::utils::api_helper::{ApiResponseType, ContentType, IsCors};
use oauth_flow::utils::crypto::CriptoHelper;
use oauth_flow::utils::injections::oauth::get_page_di::{
    GetPageAppClient, GetPageAppInitialisation,
};
use serde_json::json;

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

    run(service_fn(|event: Request| handler(&app_client, event))).await
}

pub async fn handler(
    app_client: &dyn GetPageAppInitialisation,
    event: Request,
) -> Result<impl IntoResponse, Error> {
    println!("{event:?}");

    let request = PageRequest::validate(&event);
    if let Some(request) = request {
        let page = app_client
            .query(&request)
            .await
            .ok()
            .and_then(|page| page);

        if let Some(page) = page {
            let url = format!(
                "{}://{}{}",
                event.uri().scheme().unwrap(),
                event.uri().host().unwrap(),
                event.uri().path_and_query().unwrap()
            );

            return Ok(ApiResponseType::Ok(
                CriptoHelper::from_base64(page)?.replace("#url#", &url),
                ContentType::Html,
                IsCors::Yes,
            )
            .to_response());
        }
    }

    Ok(ApiResponseType::Conflict(
        json!({ "errors": ["Cannot load UI"] }).to_string(),
        IsCors::Yes,
    )
    .to_response())
}
