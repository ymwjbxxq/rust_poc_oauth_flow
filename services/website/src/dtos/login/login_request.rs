use lambda_http::{Body, RequestExt};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder as Builder;

#[derive(Debug, Builder, Serialize, Deserialize)]
pub struct LoginRequest {
    #[builder(default, setter(into))]
    pub client_id: String,

    #[builder(default, setter(into))]
    pub host: String,
}

impl LoginRequest {
    pub fn validate(event: &http::Request<Body>) -> Option<LoginRequest> {
        let query_params = event.query_string_parameters();
        let client_id = query_params.first("client_id");
        let host = event.headers().get("Host");

        if let (Some(client_id), Some(host)) = (client_id, host) {
            return Some(
                Self::builder()
                    .client_id(client_id)
                    .host(host.to_str().unwrap())
                    .build(),
            );
        }
        None
    }
}
