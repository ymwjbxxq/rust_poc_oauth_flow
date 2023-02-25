use lambda_http::{Body, RequestExt};
use typed_builder::TypedBuilder as Builder;

use crate::utils::cookie::CookieHelper;

#[derive(Debug, Builder)]
pub struct UpdateConsentRequest {
    #[builder(default, setter(into))]
    pub is_optin: String,

    #[builder(default, setter(into))]
    pub email: String,

    #[builder(default, setter(into))]
    pub client_id: String,

    #[builder(default, setter(into))]
    pub host: String,
}

impl UpdateConsentRequest {
    pub fn validate(event: &http::Request<Body>) -> Option<UpdateConsentRequest> {
        let cookie = CookieHelper::from_http_header(event.headers()).ok();
        if let Some(cookie) = cookie {
            let is_optin = cookie.get("is_optin");
            let email = cookie.get("email");
            let query_params = event.query_string_parameters();
            let client_id = query_params.first("client_id");
            let host = event.headers().get("Host");

            if let (Some(is_optin), Some(email), Some(client_id), Some(host)) =
                (is_optin, email, client_id, host)
            {
                return Some(
                    Self::builder()
                        .is_optin(is_optin)
                        .email(email)
                        .client_id(client_id)
                        .host(host.to_str().unwrap())
                        .build(),
                );
            }
        }
        None
    }
}
