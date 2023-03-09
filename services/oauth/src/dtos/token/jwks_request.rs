use lambda_http::{Body, RequestExt};
use typed_builder::TypedBuilder as Builder;

#[derive(Debug, Builder)]
pub struct JwksRequest {
    #[builder(setter(into))]
    pub client_id: String,
}

impl JwksRequest {
    pub fn validate(event: &http::Request<Body>) -> Option<JwksRequest> {
        let path_params = event.path_parameters();
        let client_id = path_params.first("client_id");

        if let Some(client_id) = client_id {
            return Some(Self::builder().client_id(client_id).build());
        }
        None
    }
}
