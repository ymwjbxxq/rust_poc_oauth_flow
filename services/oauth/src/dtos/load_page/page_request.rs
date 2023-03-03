use lambda_http::{Body, RequestExt};
use typed_builder::TypedBuilder as Builder;

#[derive(Builder)]
pub struct PageRequest {
    #[builder(setter(into))]
    pub client_id: String,
}

impl PageRequest {
    pub fn validate(event: &http::Request<Body>) -> Option<PageRequest> {
        let query_params = event.query_string_parameters();
            let client_id = query_params.first("client_id");

            if let Some(client_id) = client_id
            {
                return Some(
                    Self::builder()
                        .client_id(client_id)
                        .build(),
                );
            }
        None
    }
}