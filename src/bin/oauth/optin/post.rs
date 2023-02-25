use cookie::Cookie;
use http::{HeaderMap, HeaderValue};
use lambda_http::{run, service_fn, Error, IntoResponse, Request, RequestExt};
use oauth_flow::dtos::optin::update_optin_request::UpdateOptInRequest;
use oauth_flow::queries::update_optin_query::UpdateOptIn;
use oauth_flow::setup_tracing;
use oauth_flow::utils::api_helper::{ApiResponseType, IsCors};
use oauth_flow::utils::cookie::CookieExt;
use oauth_flow::utils::injections::oauth::optin::post_di::{PostAppClient, PostAppInitialisation};
use serde_json::json;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_tracing();

    let config = aws_config::load_from_env().await;
    let dynamodb_client = aws_sdk_dynamodb::Client::new(&config);

    let table_name = std::env::var("TABLE_NAME").expect("TABLE_NAME must be set");
    let query = UpdateOptIn::builder()
        .table_name(table_name)
        .client(dynamodb_client)
        .build();

    let redirect_path =
        std::env::var("OAUTH_AUTHORIZE_PATH").expect("OAUTH_AUTHORIZE_PATH must be set");
    let app_client = PostAppClient::builder()
        .query(query)
        .redirect_path(redirect_path)
        .build();

    run(service_fn(|event: Request| handler(&app_client, event))).await
}

pub async fn handler(
    app_client: &dyn PostAppInitialisation,
    event: Request,
) -> Result<impl IntoResponse, Error> {
    println!("{event:?}");

    let request = UpdateOptInRequest::validate(&event);
    if let Some(request) = request {
        let is_opted_in = app_client.query(&request).await.is_ok();
        if is_opted_in {
            let cookie = Cookie::to_cookie_string(
                String::from("myOAuth"),
                HashMap::from([
                    (String::from("user"), request.user.to_owned()),
                    (String::from("is_consent"), request.is_consent),
                    (String::from("is_optin"), "true".to_owned()),
                ]),
            );
            let mut headers = HeaderMap::new();
            headers.insert(http::header::SET_COOKIE, HeaderValue::from_str(&cookie)?);
            let target = ApiResponseType::build_url_from_querystring(
               format!("https://{}{}", request.host, app_client.redirect_path()),
                event.query_string_parameters(),
            );

            return Ok(ApiResponseType::FoundWithCustomHeaders(target, IsCors::No, headers).to_response())
        }
    }

    Ok(ApiResponseType::BadRequest(
        json!({ "errors": ["Input request not valid"] }).to_string(),
        IsCors::No,
    )
    .to_response())
}
