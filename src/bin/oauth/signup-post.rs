use lambda_http::{run, service_fn, Error, IntoResponse, Request, RequestExt};
use oauth_flow::aws::client::AWSClient;
use oauth_flow::aws::client::AWSConfig;
use oauth_flow::models::user::User;
use oauth_flow::queries::add_user_query::{AddQuery, AddUser};
use oauth_flow::setup_tracing;
use oauth_flow::utils::api_helper::BodyExt;
use oauth_flow::utils::api_helper::{ApiHelper, ApiResponse, HttpStatusCode};
use oauth_flow::utils::crypto::CriptoHelper;
use serde_json::json;

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
    //register
    let result = register_user(aws_client, &event).await?;
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

async fn register_user(aws_client: &AWSClient, event: &Request) -> Result<Option<bool>, Error> {
    let query_params = event.query_string_parameters();
    let client_id = query_params.first("client_id").expect("client_id not found");

    let mut user = event.get_from_body::<User>()?.unwrap();
    user.client_id = Some(client_id.to_owned());
    user.email = Some(CriptoHelper::to_sha256_string(
        &user.email.unwrap().as_bytes(),
    ));
    user.password = Some(CriptoHelper::to_sha256_string(
        &user.password.unwrap().as_bytes(),
    ));

    AddUser::new(aws_client.dynamo_db_client.as_ref().unwrap())
        .execute(&user)
        .await?;

    Ok(Some(true))
}
