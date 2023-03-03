use lambda_http::{run, service_fn, Error, IntoResponse, Request, RequestExt};
use oauth::queries::get_csrf_query::GetCSRF;
use oauth::queries::get_csrf_query::GetCSRFRequest;
use oauth::setup_tracing;
use oauth::utils::injections::authorize::authorize_di::AuthorizeAppClient;
use oauth::utils::injections::authorize::authorize_di::AuthorizeAppInitialisation;
use shared::utils::api_helper::{ApiResponseType, IsCors};
use std::collections::HashMap;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_tracing();
    let config = aws_config::load_from_env().await;
    let dynamodb_client = aws_sdk_dynamodb::Client::new(&config);

    let table_name = std::env::var("CSRF_TABLE_NAME").expect("CSRF_TABLE_NAME must be set");
    let get_csrf_query = GetCSRF::builder()
        .table_name(table_name)
        .client(dynamodb_client)
        .build();

    let oauth_authorize_login_path = std::env::var("OAUTH_AUTHORIZE_LOGIN_PATH")
        .expect("OAUTH_AUTHORIZE_LOGIN_PATH must be set");

    // TODO take them dynamically
    let oauth_custom_optin_path =
        std::env::var("OAUTH_CUSTOM_OPTIN_PATH").expect("OAUTH_CUSTOM_OPTIN_PATH must be set");
    let oauth_custom_consent_path =
        std::env::var("OAUTH_CUSTOM_CONSENT_PATH").expect("OAUTH_CUSTOM_CONSENT_PATH must be set");
    //--TODO

    let app_client = AuthorizeAppClient::builder()
        .get_csrf_query(get_csrf_query)
        .oauth_authorize_login_path(oauth_authorize_login_path)
        .oauth_custom_consent_path(oauth_custom_consent_path)
        .oauth_custom_optin_path(oauth_custom_optin_path)
        .build();

    run(service_fn(|event: Request| handler(&app_client, event))).await
}

pub async fn handler(
    app_client: &dyn AuthorizeAppInitialisation,
    event: Request,
) -> Result<impl IntoResponse, Error> {
    println!("{event:?}");
    let query_params = event.query_string_parameters();
    let host = event
        .headers()
        .get("Host")
        .expect("Cannot find host in the Request")
        .to_str()?;

    let mut target = ApiResponseType::build_url_from_querystring(
        format!("https://{host}{}", app_client.oauth_authorize_login_path()),
        query_params.clone(),
    );

    let client_id = query_params
        .first("client_id")
        .expect("client_id is not in query string");
    let code_challenge = query_params.first("code_challenge");
    if let Some(code_challenge) = code_challenge {
        let sk = format!("code_challenge#{code_challenge}");
        let login_status = app_client
            .get_csrf_query(
                &GetCSRFRequest::builder()
                    .client_id(client_id.to_owned())
                    .sk(sk.to_owned())
                    .build(),
            )
            .await
            .ok()
            .and_then(|result| result);
        if let Some(_login_status) = login_status {
            // if login_status.is_optin {
            //     println!("is_optin is none");

            //     let target = ApiResponseType::build_url_from_querystring(
            //         format!("https://{host}{}", app_client.oauth_custom_optin_path()),
            //         query_params,
            //     );
            //     return Ok(ApiResponseType::Found(target, IsCors::Yes).to_response());
            // }

            // if login_status.is_consent {
            //     println!("is_consent is none");
            //     let target = ApiResponseType::build_url_from_querystring(
            //         format!("https://{host}{}", app_client.oauth_custom_consent_path()),
            //         query_params,
            //     );
            //     return Ok(ApiResponseType::Found(target, IsCors::Yes).to_response());
            // }

            target = ApiResponseType::build_url_from_hashmap(
                query_params
                    .first("redirect_uri")
                    .expect("redirect_uri is not in query string")
                    .to_owned(),
                HashMap::from([
                    ("client_id", client_id),
                    ("code", &Uuid::new_v4().to_string()),
                    (
                        "state",
                        query_params
                            .first("state")
                            .expect("state is not in query string"),
                    ),
                    ("code_challenge", code_challenge),
                    (
                        "redirect_uri",
                        query_params
                            .first("redirect_uri")
                            .expect("redirect_uri is not in query string"),
                    ),
                ]),
            );
        }
    }

    Ok(ApiResponseType::Found(target, IsCors::Yes).to_response())
}
