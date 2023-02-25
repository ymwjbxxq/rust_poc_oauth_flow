use cookie::Cookie;
use http::HeaderMap;
use http::HeaderValue;
use lambda_http::{run, service_fn, Error, IntoResponse, Request, RequestExt};
use oauth_flow::setup_tracing;
use oauth_flow::utils::api_helper::ApiResponseType;
use oauth_flow::utils::api_helper::IsCors;
use oauth_flow::utils::cookie::CookieExt;
use oauth_flow::utils::cookie::CookieHelper;
use std::collections::HashMap;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_tracing();

    run(service_fn(handler)).await
}

pub async fn handler(event: Request) -> Result<impl IntoResponse, Error> {
    println!("{event:?}");
    let query_params = event.query_string_parameters();
    let host = event
        .headers()
        .get("Host")
        .expect("Cannot find host in the Request")
        .to_str()?;

    let redirect_path = std::env::var("OAUTH_AUTHORIZE_LOGIN_PATH")
        .expect("OAUTH_AUTHORIZE_LOGIN_PATH must be set");
    let mut target = ApiResponseType::build_url_from_querystring(
        format!("https://{host}{redirect_path}"),
        query_params.clone(),
    );

    let mut headers = HeaderMap::new();
    let cookie = CookieHelper::from_http_header(event.headers());
    if cookie.is_ok() {
        let mut cookie = cookie.unwrap();
        if cookie.get("is_optin").is_none() {
            let redirect_path = std::env::var("OAUTH_CUSTOM_OPTIN_PATH")
                .expect("OAUTH_CUSTOM_OPTIN_PATH must be set");

            let target = ApiResponseType::build_url_from_querystring(
                format!("https://{host}{redirect_path}"),
                query_params,
            );
            return Ok(ApiResponseType::Found(target, IsCors::No).to_response());
        }

        if cookie.get("is_consent").is_none() {
            let redirect_path = std::env::var("OAUTH_CUSTOM_CONSENT_PATH")
                .expect("OAUTH_CUSTOM_CONSENT_PATH must be set");

            let target = ApiResponseType::build_url_from_querystring(
                format!("https://{host}{redirect_path}"),
                query_params,
            );
            return Ok(ApiResponseType::Found(target, IsCors::No).to_response());
        }

        cookie.insert(
            "code_challenge".to_owned(),
            query_params
                .first("code_challenge")
                .expect("code_challenge is not in query string")
                .to_owned(),
        );
        let cookie = Cookie::to_cookie_string(String::from("myOAuth"), cookie);

        headers.insert(http::header::SET_COOKIE, HeaderValue::from_str(&cookie)?);

        target = ApiResponseType::build_url_from_hashmap(
            query_params
                .first("redirect_uri")
                .expect("redirect_uri is not in query string")
                .to_owned(),
            HashMap::from([
                (
                    "client_id",
                    query_params
                        .first("client_id")
                        .expect("client_id is not in query string"),
                ),
                ("code", &Uuid::new_v4().to_string()),
                (
                    "state",
                    query_params
                        .first("state")
                        .expect("state is not in query string"),
                ),
                (
                    "code_challenge",
                    query_params
                        .first("code_challenge")
                        .expect("code_challenge is not in query string"),
                ),
                (
                    "redirect_uri",
                    query_params
                        .first("redirect_uri")
                        .expect("redirect_uri is not in query string"),
                ),
            ]),
        );
    }

    Ok(ApiResponseType::FoundWithCustomHeaders(target, IsCors::No, headers).to_response())
}
