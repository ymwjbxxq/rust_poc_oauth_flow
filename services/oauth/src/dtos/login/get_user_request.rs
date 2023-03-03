use lambda_http::{Body, RequestExt};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder as Builder;

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    pub email: String,

    pub password: String,
}

#[derive(Debug, Builder)]
pub struct GetUserRequest {
    #[builder(default, setter(into))]
    pub email: String,

    #[builder(default, setter(into))]
    pub password: String,

    #[builder(default, setter(into))]
    pub client_id: String,

    #[builder(default, setter(into))]
    pub code_challenge: String,

    #[builder(default, setter(into))]
    pub host: String,
}

impl GetUserRequest {
    pub fn validate(event: &http::Request<Body>) -> Option<GetUserRequest> {
        let user = event.payload::<LoginRequest>().ok().and_then(|user| user);
        if let Some(user) = user {
            let query_params = event.query_string_parameters();
            let client_id = query_params.first("client_id");
            let code_challenge = query_params.first("code_challenge");
            let host = event.headers().get("Host");

            if let (Some(client_id), Some(host), Some(code_challenge)) =
                (client_id, host, code_challenge)
            {
                return Some(
                    Self::builder()
                        .email(user.email)
                        .password(user.password)
                        .client_id(client_id)
                        .code_challenge(code_challenge)
                        .host(host.to_str().unwrap())
                        .build(),
                );
            }
        }
        None
    }
}
