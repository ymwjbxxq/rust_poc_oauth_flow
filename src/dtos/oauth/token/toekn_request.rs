use crate::utils::cookie::CookieHelper;
use lambda_http::{Body, RequestExt};
use typed_builder::TypedBuilder as Builder;

#[derive(Debug, Builder)]
pub struct TokenRequest {
    #[builder(setter(into))]
    pub code_verifier: String,

    #[builder(setter(into))]
    pub code_challenge: String,

    #[builder(setter(into))]
    pub user: String,

    #[builder(setter(into))]
    pub client_id: String,
}

impl TokenRequest {
    pub fn validate(event: &http::Request<Body>) -> Option<TokenRequest> {
        let cookie = CookieHelper::from_http_header(event.headers()).ok();
        if let Some(cookie) = cookie {
            let code_challenge = cookie.get("code_challenge");
            let user = cookie.get("user");
            let query_params = event.query_string_parameters();
            let code_verifier = query_params.first("code_verifier");
            let client_id = query_params.first("client_id");

            if let (Some(code_challenge), Some(code_verifier),  Some(user), Some(client_id)) = (code_challenge, code_verifier, user, client_id) {
                return Some(
                    Self::builder()
                        .code_verifier(code_verifier)
                        .code_challenge(code_challenge)
                        .user(user)
                        .client_id(client_id)
                        .build(),
                );
            }
        }
        None
    }
}
