use cookie::Cookie;
use lambda_http::{run, service_fn, Error, IntoResponse, Request, RequestExt};
use oauth_flow::queries::update_consent_query::UpdateConsentRequest;
use oauth_flow::queries::update_consent_query::UpdateConsent;
use oauth_flow::setup_tracing;
use oauth_flow::utils::api_helper::{ApiHelper, ApiResponse, HttpStatusCode};
use oauth_flow::utils::cookie::CookieExt;
use oauth_flow::utils::cookie::CookieHelper;
use oauth_flow::utils::injections::oauth::consent::post_di::{
    PostAppClient, PostAppInitialisation,
};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_tracing();

    let config = aws_config::load_from_env().await;
    let dynamodb_client = aws_sdk_dynamodb::Client::new(&config);

    let table_name = std::env::var("TABLE_NAME").expect("TABLE_NAME must be set");
    let query = UpdateConsent::builder()
        .table_name(table_name)
        .client(dynamodb_client.clone())
        .build();

    let redirect_path =
        std::env::var("OAUTH_AUTHORIZE_PATH").expect("OAUTH_AUTHORIZE_PATH must be set");
    let app_client = PostAppClient::builder()
        .query(query)
        .redirect_path(redirect_path)
        .build();

    run(service_fn(|event: Request| execute(&app_client, event))).await
}

pub async fn execute(
    app_client: &dyn PostAppInitialisation,
    event: Request,
) -> Result<impl IntoResponse, Error> {
    println!("{event:?}");

    let cookie = CookieHelper::from_http_header(event.headers())?;

    let is_optin = cookie
        .get("is_optin")
        .expect("is_optin must be set at this point");
    let email = cookie
        .get("email")
        .expect("email must be set at this point");

    let query_params = event.query_string_parameters();
    let client_id = query_params
        .first("client_id")
        .expect("client_id not found");

    let request = UpdateConsentRequest::builder()
        .client_id(client_id)
        .email(email)
        .is_consent(true)
        .build();

    app_client.query(&request).await?;

    let host = event.headers().get("Host").unwrap().to_str().unwrap();

    let mut headers = HashMap::new();
    headers.insert(
        http::header::SET_COOKIE,
        Cookie::to_cookie_string(
            String::from("myOAuth"),
            HashMap::from([
                (String::from("email"), email.to_owned()),
                (String::from("is_consent"), true.to_string()),
                (String::from("is_optin"), is_optin.to_string()),
            ]),
        ),
    );
    headers.insert(http::header::CONTENT_TYPE, "application/json".to_string());
    headers.insert(
        http::header::LOCATION,
        ApiHelper::build_url_from_querystring(
            format!("https://{host}{}", app_client.redirect_path()),
            query_params,
        ),
    );

    Ok(ApiHelper::response(ApiResponse {
        status_code: HttpStatusCode::Found,
        body: None,
        headers: Some(headers),
    }))
}
