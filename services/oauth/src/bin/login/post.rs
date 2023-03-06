use lambda_http::{run, service_fn, Error, IntoResponse, Request, RequestExt};
use oauth::dtos::login::login_user_request::LoginUserRequest;
use oauth::queries::add_csrf_query::{AddCSRF, AddCSRFRequest};
use oauth::queries::login_user_query::LoginUser;
use oauth::setup_tracing;
use oauth::utils::injections::login::post_di::{PostAppClient, PostAppInitialisation};
use serde_json::json;
use shared::utils::api_helper::{ApiResponseType, IsCors};

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_tracing();

    let config = aws_config::load_from_env().await;
    let dynamodb_client = aws_sdk_dynamodb::Client::new(&config);

    let table_name = std::env::var("USER_TABLE_NAME").expect("USER_TABLE_NAME must be set");
    let get_user_query = LoginUser::builder()
        .table_name(table_name)
        .client(dynamodb_client.clone())
        .build();

    let table_name = std::env::var("CSRF_TABLE_NAME").expect("CSRF_TABLE_NAME must be set");
    let add_csrf_query = AddCSRF::builder()
        .table_name(table_name.to_owned())
        .client(dynamodb_client)
        .build();

    let redirect_path =
        std::env::var("OAUTH_AUTHORIZE_PATH").expect("OAUTH_AUTHORIZE_PATH must be set");
    let app_client = PostAppClient::builder()
        .get_user_query(get_user_query)
        .add_csrf_query(add_csrf_query)
        .redirect_path(redirect_path)
        .build();

    run(service_fn(|event: Request| handler(&app_client, event))).await
}

pub async fn handler(
    app_client: &dyn PostAppInitialisation,
    event: Request,
) -> Result<impl IntoResponse, Error> {
    println!("{event:?}");
    let request = LoginUserRequest::validate(&event);
    if let Some(request) = request {
        let user = app_client
            .get_user_query(&request)
            .await
            .ok()
            .and_then(|result| result);
        if let Some(user) = user {
            let result = app_client
                .add_csrf_query(
                    &AddCSRFRequest::builder()
                        .client_id(request.client_id.to_owned())
                        .sk(format!("code_challenge####{}", request.code_challenge))
                        .data(Some(user.user))
                        .build(),
                )
                .await;
            if result.is_ok() {
                let target = ApiResponseType::build_url_from_querystring(
                    format!("https://{}{}", request.host, app_client.redirect_path(),),
                    event.query_string_parameters(),
                );

                return Ok(ApiResponseType::Found(target, IsCors::Yes).to_response());
            }
        }
    }

    Ok(ApiResponseType::NoContent(
        json!({ "errors": ["User not found"] }).to_string(),
        IsCors::Yes,
    )
    .to_response())
}
