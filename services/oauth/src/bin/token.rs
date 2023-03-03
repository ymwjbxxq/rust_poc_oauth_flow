use chrono::{Duration, Utc};
use lambda_http::{run, service_fn, Error, IntoResponse, Request};
use oauth::dtos::token::toekn_request::TokenRequest;
use oauth::queries::delete_csrf_query::{DeleteCSRF, DeleteCSRFRequest};
use oauth::queries::get_csrf_query::{GetCSRF, GetCSRFRequest};
use oauth::setup_tracing;
use oauth::utils::injections::token::token_di::{ToeknAppClient, ToeknAppInitialisation};
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
        .client(dynamodb_client)
        .build();

    let app_client = ToeknAppClient::builder()
        .get_csrf_query(get_csrf_query)
        .delete_csrf_query(delete_csrf_query)
        .build();

    run(service_fn(|event: Request| handler(&app_client, event))).await
}

pub async fn handler(
    app_client: &dyn ToeknAppInitialisation,
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
                        "code_challenge#{}",
                        request.code_verifier.to_owned()
                    ))
                    .build(),
            )
            .await
            .ok()
            .and_then(|result| result);

        if let Some(csrf_code_challange) = csrf_code_challange {
            let result = app_client
                .delete_csrf_query(
                    &DeleteCSRFRequest::builder()
                        .client_id(request.client_id.to_owned())
                        .sk(format!(
                            "code_challenge#{}",
                            request.code_verifier.to_owned()
                        ))
                        .build(),
                )
                .await;
            if result.is_ok() {
                let claims = Claims::builder()
                    .sub(csrf_code_challange.data.unwrap())
                    .company(request.client_id)
                    .exp((Utc::now() + Duration::minutes(10)).timestamp())
                    .build();

                let token = Jwt::new("private_key")
                    .encode(&claims)
                    .ok()
                    .and_then(|token| token);

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
