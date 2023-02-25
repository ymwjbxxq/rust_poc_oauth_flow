use cookie::Cookie;
use lambda_http::{run, service_fn, Error, IntoResponse, Request, RequestExt};
use oauth_flow::models::user::User;
use oauth_flow::queries::get_user_query::{GetUser, GetUserRequest};
use oauth_flow::setup_tracing;
use oauth_flow::utils::api_helper::{ApiHelper, ApiResponse, HttpStatusCode};
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

    run(service_fn(|event: Request| execute(&app_client, event))).await
}

pub async fn execute(
    app_client: &dyn PostAppInitialisation,
    event: Request,
) -> Result<impl IntoResponse, Error> {
    println!("{event:?}");
    let result = login(app_client, &event)
        .await
        .ok()
        .and_then(|result| result);
    
    let query_params = event.query_string_parameters();
    return Ok(match result {
        Some(user) => {
            let host = event
                .headers()
                .get("Host")
                .expect("Cannot find host in the Request")
                .to_str()?;

            let mut headers = HashMap::new();
            headers.insert(
                http::header::SET_COOKIE,
                Cookie::to_cookie_string(
                    String::from("myOAuth"),
                    HashMap::from([
                        (String::from("email"), user.email.unwrap()),
                        (
                            String::from("is_consent"),
                            user.is_consent.unwrap().to_string(),
                        ),
                        (String::from("is_optin"), user.is_optin.unwrap().to_string()),
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

            ApiHelper::response(ApiResponse {
                status_code: HttpStatusCode::Found,
                body: None,
                headers: Some(headers),
            })
        }
        None => ApiHelper::response(ApiResponse {
            status_code: HttpStatusCode::NotFound,
            body: Some(json!({ "message": "User not found" }).to_string()),
            headers: None,
        }),
    });
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
