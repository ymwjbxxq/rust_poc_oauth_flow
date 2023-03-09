// use http::HeaderMap;
// use http::HeaderValue;
// use lambda_http::{run, service_fn, Error, IntoResponse, Request, RequestExt};
// use oauth::dtos::consent::update_consent_request::UpdateConsentRequest;
// use oauth::queries::update_consent_query::UpdateConsent;
// use oauth::setup_tracing;
// use oauth::utils::injections::consent::post_di::{PostAppClient, PostAppInitialisation};
// use serde_json::json;
// use shared::utils::api_helper::{ApiResponseType, IsCors};
// use std::collections::HashMap;

// #[tokio::main]
// async fn main() -> Result<(), Error> {
//     setup_tracing();

//     let config = aws_config::load_from_env().await;
//     let dynamodb_client = aws_sdk_dynamodb::Client::new(&config);

//     let table_name = std::env::var("TABLE_NAME").expect("TABLE_NAME must be set");
//     let query = UpdateConsent::builder()
//         .table_name(table_name)
//         .client(dynamodb_client)
//         .build();

//     let redirect_path =
//         std::env::var("OAUTH_AUTHORIZE_PATH").expect("OAUTH_AUTHORIZE_PATH must be set");
//     let app_client = PostAppClient::builder()
//         .query(query)
//         .redirect_path(redirect_path)
//         .build();

//     run(service_fn(|event: Request| handler(&app_client, event))).await
// }

// pub async fn handler(
//     app_client: &dyn PostAppInitialisation,
//     event: Request,
// ) -> anyhow::Result<impl IntoResponse> {
//     println!("{event:?}");

//     let request = UpdateConsentRequest::validate(&event);
//     if let Some(request) = request {
//         let is_consent_updated = app_client.query(&request).await.is_ok();
//         if is_consent_updated {
//             // let cookie = Cookie::to_cookie_string(
//             //     String::from("myOAuth"),
//             //     HashMap::from([
//             //         (String::from("user"), request.user.to_owned()),
//             //         (String::from("is_consent"), "true".to_owned()),
//             //         (String::from("is_optin"), request.is_optin),
//             //     ]),
//             // );
//             // let mut headers = HeaderMap::new();
//             // headers.insert(http::header::SET_COOKIE, HeaderValue::from_str(&cookie)?);
//             // let target = ApiResponseType::build_url_from_querystring(
//             //     format!("https://{}{}", request.host, app_client.redirect_path()),
//             //     event.query_string_parameters(),
//             // );

//             // return Ok(
//             //     ApiResponseType::FoundWithCustomHeaders(target, IsCors::Yes, headers).to_response(),
//             // );
//         }
//     }

//     Ok(ApiResponseType::BadRequest(
//         json!({ "errors": ["Input request not valid"] }).to_string(),
//         IsCors::Yes,
//     )
//     .to_response())
// }
