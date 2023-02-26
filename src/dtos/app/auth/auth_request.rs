use crate::utils::cookie::CookieHelper;
use lambda_http::{Body, RequestExt};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder as Builder;

#[derive(Debug, Builder, Serialize, Deserialize)]
pub struct AuthRequest {
    #[builder(default, setter(into))]
    pub client_id: String,

    #[builder(default, setter(into))]
    pub host: String,

    #[builder(default, setter(into))]
    pub cookie_state: String,

    #[builder(default, setter(into))]
    pub querystring_state: String,

    #[builder(default, setter(into))]
    pub code: String,

    #[builder(default, setter(into))]
    pub code_verifier: String,

    #[builder(default, setter(into))]
    pub redirect_uri: String,
}

impl AuthRequest {
    pub fn validate(event: &http::Request<Body>) -> Option<AuthRequest> {
        let cookie = CookieHelper::from_http_header(event.headers()).ok();
        if let Some(cookie) = cookie {
            let cookie_state = cookie.get("state");

            let query_params = event.query_string_parameters();
            let client_id = query_params.first("client_id");
            let querystring_state = query_params.first("state");
            let code = query_params.first("code");
            let code_verifier = query_params.first("code_verifier");
            let redirect_uri = query_params.first("redirect_uri");
            let host = event.headers().get("Host");

            if let (
                Some(cookie_state),
                Some(client_id),
                Some(querystring_state),
                Some(host),
                Some(code),
                Some(code_verifier),
                Some(redirect_uri),
            ) = (
                cookie_state,
                client_id,
                querystring_state,
                host,
                code,
                code_verifier,
                redirect_uri,
            ) {
                return Some(
                    Self::builder()
                        .client_id(client_id)
                        .host(host.to_str().unwrap())
                        .cookie_state(cookie_state)
                        .querystring_state(querystring_state)
                        .code(code)
                        .code_verifier(code_verifier)
                        .redirect_uri(redirect_uri)
                        .build(),
                );
            }
        }

        None
    }
}
