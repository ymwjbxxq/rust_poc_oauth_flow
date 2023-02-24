use cookie::Cookie;
use lambda_http::{run, service_fn, Error, IntoResponse, Request, RequestExt};
use oauth_flow::aws::client::AWSClient;
use oauth_flow::aws::client::AWSConfig;
use oauth_flow::queries::update_optin_query::{UpdateOptIn, UpdateOptInCommand, UpdateOptInQuery};
use oauth_flow::setup_tracing;
use oauth_flow::utils::api_helper::{ApiHelper, ApiResponse, HttpStatusCode};
use oauth_flow::utils::cookie::CookieExt;
use oauth_flow::utils::cookie::CookieHelper;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_tracing();

    let config = aws_config::load_from_env().await;
    let aws_client = AWSConfig::set_config(config);
    let dynamo_db_client = aws_client.dynamo_client();
    let aws_client = AWSClient {
        dynamo_db_client: Some(dynamo_db_client),
        s3_client: None,
    };

    run(service_fn(|event: Request| {
        execute(&aws_client, event)
    }))
    .await
}

pub async fn execute(
    aws_client: &AWSClient,
    event: Request,
) -> Result<impl IntoResponse, Error> {
    println!("{event:?}");
    let cookie = CookieHelper::from_http_header(event.headers())?;
    let is_consent = cookie
        .get("is_consent")
        .expect("is_consent must be set at this point");
    let email = cookie
        .get("email")
        .expect("email must be set at this point");
    let query_params = event.query_string_parameters();
    let client_id = query_params.first("client_id").expect("client_id not found");

    UpdateOptInQuery::new(aws_client.dynamo_db_client.as_ref().unwrap())
        .execute(UpdateOptInCommand {
            client_id: client_id.to_owned(),
            email: email.to_owned(),
            is_optin: true,
        })
        .await?;

    let redirect_path =
        std::env::var("OAUTH_AUTHORIZE_PATH").expect("OAUTH_AUTHORIZE_PATH must be set");
    let host = event.headers().get("Host").unwrap().to_str().unwrap();

    let mut headers = HashMap::new();
    headers.insert(
        http::header::SET_COOKIE,
        Cookie::to_cookie_string(
            String::from("myOAuth"),
            HashMap::from([
                (String::from("email"), email.to_owned()),
                (String::from("is_consent"), is_consent.to_string()),
                (String::from("is_optin"), true.to_string()),
            ]),
        ),
    );
    headers.insert(http::header::CONTENT_TYPE, "application/json".to_string());
    headers.insert(
        http::header::LOCATION,
        ApiHelper::build_url_from_querystring(
            format!("https://{host}{redirect_path}"),
            query_params,
        ),
    );

    Ok(ApiHelper::response(ApiResponse {
        status_code: HttpStatusCode::Found,
        body: None,
        headers: Some(headers),
    }))
}
