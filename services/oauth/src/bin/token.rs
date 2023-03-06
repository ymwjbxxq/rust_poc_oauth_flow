use chrono::{Duration, Utc};
use lambda_http::{run, service_fn, Error, IntoResponse, Request};
use oauth::dtos::token::get_user_request::GetUserRequest;
use oauth::dtos::token::token_request::TokenRequest;
use oauth::queries::delete_csrf_query::{DeleteCSRF, DeleteCSRFRequest};
use oauth::queries::get_csrf_query::{GetCSRF, GetCSRFRequest};
use oauth::queries::get_user_query::GetUser;
use oauth::setup_tracing;
use oauth::utils::injections::token::token_di::{TokenAppClient, TokenAppInitialisation};
use serde_json::json;
use shared::utils::api_helper::{ApiResponseType, ContentType, IsCors};
use shared::utils::jwt::{Claims, Jwt};

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_tracing();

    let config = aws_config::load_from_env().await;
    let dynamodb_client = aws_sdk_dynamodb::Client::new(&config);

    let table_name = std::env::var("CSRF_TABLE_NAME").expect("CSRF_TABLE_NAME must be set");
    let get_csrf_query = GetCSRF::builder()
        .table_name(table_name.to_string())
        .client(dynamodb_client.clone())
        .build();

    let delete_csrf_query = DeleteCSRF::builder()
        .table_name(table_name)
        .client(dynamodb_client.clone())
        .build();

    let table_name = std::env::var("USER_TABLE_NAME").expect("USER_TABLE_NAME must be set");
    let get_user_query = GetUser::builder()
        .table_name(table_name)
        .client(dynamodb_client.clone())
        .build();

    let app_client = TokenAppClient::builder()
        .get_user_query(get_user_query)
        .get_csrf_query(get_csrf_query)
        .delete_csrf_query(delete_csrf_query)
        .build();

    run(service_fn(|event: Request| handler(&app_client, event))).await
}

pub async fn handler<'a>(
    app_client: &dyn TokenAppInitialisation,
    event: Request,
) -> Result<impl IntoResponse, Error> {
    println!("{event:?}");

    let mut messages: Vec<String> = Vec::new();
    let request = TokenRequest::validate(&event);
    if let Some(request) = request {
        let csrf_code_challange = app_client
            .get_csrf_query(
                &GetCSRFRequest::builder()
                    .client_id(request.client_id.to_owned())
                    .sk(format!(
                        "code_challenge####{}",
                        request.code_verifier.to_owned()
                    ))
                    .build(),
            )
            .await
            .ok()
            .and_then(|result| result);

        if let Some(csrf_code_challange) = csrf_code_challange {

            let user = csrf_code_challange
                .data
                .unwrap_or("none####none".to_owned());
            let email = user
                .split("####")
                .collect::<Vec<&str>>()[0]
                .to_owned();
            let (delete_csrf, user) = token_preparation(app_client, &request, &user).await;
            if delete_csrf.is_ok() && user.is_some() {
                let token = generate_token(email, request);
                if let Some(token) = token {
                    return Ok(ApiResponseType::Ok(
                        json!({ "token": token }).to_string(),
                        ContentType::Json,
                        IsCors::Yes,
                    )
                    .to_response());
                } else {
                    messages.push("problem creating token".to_string());
                }
            } else {
                messages.push("problem deleting csrf".to_string());
            }
        } else {
            messages.push("csrf not found".to_string());
        }
    } else {
        messages.push("Invalid request".to_string());
    }

    println!("token Unauthorized");
    Ok(
        ApiResponseType::Forbidden(json!({ "errors": messages }).to_string(), IsCors::Yes)
            .to_response(),
    )
}

async fn token_preparation(app_client: &dyn TokenAppInitialisation, request: &TokenRequest, user: &String) -> (Result<(), shared::error::ApplicationError>, Option<oauth::models::user::User>) {
    let delete_csrf = app_client
        .delete_csrf_query(
            &DeleteCSRFRequest::builder()
                .client_id(request.client_id.to_owned())
                .sk(format!(
                    "code_challenge####{}",
                    request.code_verifier.to_owned()
                ))
                .build(),
        )
        .await;

    let user = app_client
        .get_user_query(
            &GetUserRequest::builder()
                .client_id(request.client_id.to_owned())
                .user(user.to_string())
                .build(),
        )
        .await
        .ok()
        .and_then(|result| result);
    (delete_csrf, user)
}

fn generate_token(email: String, request: TokenRequest) -> Option<String> {
    let claims = Claims::builder()
        .iss("https://www.authservice.com/")
        .sub(format!("authservice|{}", email))
        .azp(request.client_id)
        .exp((Utc::now() + Duration::minutes(10)).timestamp())
        .build();

    let token = Jwt::new("private_key")
        .encode(&claims)
        .ok()
        .and_then(|token| token);
    token
}
