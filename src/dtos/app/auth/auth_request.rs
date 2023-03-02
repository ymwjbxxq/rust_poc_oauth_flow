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
    pub state: String,

    #[builder(default, setter(into))]
    pub code: String,

    #[builder(default, setter(into))]
    pub code_challenge: String,

    #[builder(default, setter(into))]
    pub redirect_uri: String,
}

impl AuthRequest {
    pub fn validate(event: &http::Request<Body>) -> Option<AuthRequest> {
        let query_params = event.query_string_parameters();
        let client_id = query_params.first("client_id");
        let state = query_params.first("state");
        let code = query_params.first("code");
        let code_challenge = query_params.first("code_challenge");
        let redirect_uri = query_params.first("redirect_uri");
        let host = event.headers().get("Host");

        if let (
            Some(client_id),
            Some(state),
            Some(host),
            Some(code),
            Some(redirect_uri),
            Some(code_challenge),
        ) = (client_id, state, host, code, redirect_uri, code_challenge)
        {
            return Some(
                Self::builder()
                    .client_id(client_id)
                    .host(host.to_str().unwrap())
                    .state(state)
                    .code(code)
                    .code_challenge(code_challenge)
                    .redirect_uri(redirect_uri)
                    .build(),
            );
        }

        None
    }
}
