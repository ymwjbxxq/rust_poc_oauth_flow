use lambda_http::{run, service_fn, Error, IntoResponse, Request};
use oauth::dtos::signup::singup_request::SignUpRequest;
use oauth::queries::user::add_user_query::AddUser;
use oauth::setup_tracing;
use oauth::utils::injections::signup::post_di::{PostAppClient, PostAppInitialisation};
use serde_json::json;
use shared::utils::api_helper::{ApiResponseType, IsCors};

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
) -> anyhow::Result<impl IntoResponse> {
    println!("{event:?}");

    let request = SignUpRequest::validate(&event);
    if let Some(request) = request {
        let is_registred = app_client.query(&request).await.is_ok();
        if is_registred {
            return Ok(ApiResponseType::Created(
                json!({ "message": ["We sent you an email please confirm your email."] })
                    .to_string(),
                IsCors::Yes,
            )
            .to_response());
        }
    }

    Ok(ApiResponseType::BadRequest(
        json!({ "errors": ["Input request not valid"] }).to_string(),
        IsCors::Yes,
    )
    .to_response())
}
