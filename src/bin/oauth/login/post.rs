use cookie::Cookie;
use http::{HeaderMap, HeaderValue};
use lambda_http::{run, service_fn, Error, IntoResponse, Request, RequestExt};
use oauth_flow::models::user::User;
use oauth_flow::queries::get_user_query::{GetUser, GetUserRequest};
use oauth_flow::setup_tracing;
use oauth_flow::utils::api_helper::{ApiResponseType, IsCors};
use oauth_flow::utils::cookie::CookieExt;
use oauth_flow::utils::injections::oauth::login::post_di::{PostAppClient, PostAppInitialisation};
use serde_json::json;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_tracing();

    let config = aws_config::load_from_env().await;
    let dynamodb_client = aws_sdk_dynamodb::Client::new(&config);

    let table_name = std::env::var("TABLE_NAME").expect("TABLE_NAME must be set");
    let query = GetUser::builder()
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
    let user = login(app_client, &event)
        .await
        .ok()
        .and_then(|result| result);

    let host = event
        .headers()
        .get("Host");

    if let (Some(user), Some(host)) = (user, host) {
        let cookie = Cookie::to_cookie_string(
            String::from("myOAuth"),
            HashMap::from([
                (String::from("email"), user.email.unwrap()),
                (
                    String::from("is_consent"),
                    user.is_consent.unwrap().to_string(),
                ),
                (String::from("is_optin"), user.is_optin.unwrap().to_string()),
            ]),
        );
        let mut headers = HeaderMap::new();
        headers.insert(http::header::SET_COOKIE, HeaderValue::from_str(&cookie)?);

        let target = ApiResponseType::build_url_from_querystring(
            format!("https://{}{}", host.to_str()?, app_client.redirect_path()),
            event.query_string_parameters(),
        );

        return Ok(
            ApiResponseType::FoundWithCustomHeaders(target, IsCors::No, headers).to_response(),
        );
    }

    Ok(ApiResponseType::NoContent(
        json!({ "errors": ["User not found"] }).to_string(),
        IsCors::No,
    )
    .to_response())
}

async fn login(
    app_client: &dyn PostAppInitialisation,
    event: &Request,
) -> Result<Option<User>, Error> {
    let user = event.payload::<User>()?.unwrap();
    let query_params = event.query_string_parameters();
    let client_id = query_params
        .first("client_id")
        .expect("client_id not found");

    let request = GetUserRequest::builder()
        .client_id(client_id)
        .email(user.email.unwrap())
        .build();

    let user = app_client.query(&request).await?;

    Ok(user)
}
