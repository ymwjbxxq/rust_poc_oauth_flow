use lambda_http::{run, service_fn, Error, IntoResponse, Request, RequestExt};
use oauth_flow::models::user::User;
use oauth_flow::queries::add_user_query::AddUser;
use oauth_flow::setup_tracing;
use oauth_flow::utils::api_helper::{ApiHelper, ApiResponse, HttpStatusCode};
use oauth_flow::utils::crypto::CriptoHelper;
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

    run(service_fn(|event: Request| execute(&app_client, event))).await
}

pub async fn execute(
    app_client: &dyn PostAppInitialisation,
    event: Request,
) -> Result<impl IntoResponse, Error> {
    println!("{event:?}");
    //register
    let result = register_user(app_client, &event)
        .await
        .ok()
        .map(|result| result.is_some());

    Ok(match result {
        Some(_) => ApiHelper::response(ApiResponse {
            status_code: HttpStatusCode::Created,
            body: Some(
                json!({ "message": "We sent you an email please confirm your email." }).to_string(),
            ),
            headers: None,
        }),
        None => ApiHelper::response(ApiResponse {
            status_code: HttpStatusCode::InternalServerError,
            body: Some(json!({ "message": "Cannot add user" }).to_string()),
            headers: None,
        }),
    })
}

async fn register_user(
    app_client: &dyn PostAppInitialisation,
    event: &Request,
) -> Result<Option<bool>, Error> {
    let query_params = event.query_string_parameters();
    let client_id = query_params
        .first("client_id")
        .expect("client_id not found");

    let mut user = event.payload::<User>()?.unwrap();

    user.client_id = Some(client_id.to_owned());
    user.email = Some(CriptoHelper::to_sha256_string(
        &user.email.unwrap().as_bytes(),
    ));
    user.password = Some(CriptoHelper::to_sha256_string(
        &user.password.unwrap().as_bytes(),
    ));

    app_client.query(&user).await?;

    Ok(Some(true))
}
