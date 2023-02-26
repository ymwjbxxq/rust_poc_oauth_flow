use lambda_http::{Body, RequestExt};
use typed_builder::TypedBuilder as Builder;
use crate::utils::cookie::CookieHelper;

#[derive(Debug, Builder)]
pub struct UpdateOptInRequest {
    #[builder(setter(into))]
    pub client_id: String,

    #[builder(setter(into))]
    pub user: String,

    #[builder(setter(into))]
    pub is_consent: String,

    #[builder(default, setter(into))]
    pub host: String,
}

impl UpdateOptInRequest {
    pub fn validate(event: &http::Request<Body>) -> Option<UpdateOptInRequest> {
        let cookie = CookieHelper::from_http_header(event.headers()).ok();
        if let Some(cookie) = cookie {
            let is_consent = cookie.get("is_consent");
            let user = cookie.get("user");
            let query_params = event.query_string_parameters();
            let client_id = query_params.first("client_id");
            let host = event.headers().get("Host");

            if let (Some(is_consent), Some(user), Some(client_id), Some(host)) =
                (is_consent, user, client_id, host)
            {
                return Some(
                    Self::builder()
                        .is_consent(is_consent)
                        .user(user)
                        .client_id(client_id)
                        .host(host.to_str().unwrap())
                        .build(),
                );
            }
        }
        None
    }
}
