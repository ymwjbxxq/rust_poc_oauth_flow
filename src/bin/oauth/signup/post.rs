use lambda_http::{run, service_fn, Error, IntoResponse, Request};
use oauth_flow::dtos::signup::singup_request::SignUpRequest;
use oauth_flow::queries::add_user_query::AddUser;
use oauth_flow::setup_tracing;
use oauth_flow::utils::api_helper::{ApiResponseType, IsCors};
use oauth_flow::utils::injections::oauth::signup::post_di::{PostAppClient, PostAppInitialisation};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_tracing();

    let config = aws_config::load_from_env().await;
    let dynamodb_client = aws_sdk_dynamodb::Client::new(&config);

    let table_name = std::env::var("TABLE_NAME").expect("TABLE_NAME must be set");
    let query = AddUser::builder()
        .table_name(table_name)
        .client(dynamodb_client)
        .build();

    let app_client = PostAppClient::builder().query(query).build();

    run(service_fn(|event: Request| handler(&app_client, event))).await
}

pub async fn handler(
    app_client: &dyn PostAppInitialisation,
    event: Request,
) -> Result<impl IntoResponse, Error> {
    println!("{event:?}");

    let request = SignUpRequest::validate(&event);
    if let Some(request) = request {
        let is_registred = app_client.query(&request).await.is_ok();
        if is_registred {
            return Ok(ApiResponseType::Created(
                json!({ "message": ["We sent you an email please confirm your email."] })
                    .to_string(),
                IsCors::No,
            )
            .to_response());
        }
    }

    Ok(ApiResponseType::BadRequest(
        json!({ "errors": ["Input request not valid"] }).to_string(),
        IsCors::No,
    )
    .to_response())
}
