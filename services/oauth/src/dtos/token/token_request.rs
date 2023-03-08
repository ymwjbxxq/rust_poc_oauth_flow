use lambda_http::{Body, RequestExt};
use typed_builder::TypedBuilder as Builder;

#[derive(Debug, Builder)]
pub struct TokenRequest {
    #[builder(setter(into))]
    pub code_verifier: String,

    #[builder(setter(into))]
    pub client_id: String,

     #[builder(setter(into))]
    pub audience: String,
}

impl TokenRequest {
    pub fn validate(event: &http::Request<Body>) -> Option<TokenRequest> {
        let query_params = event.query_string_parameters();
        let code_verifier = query_params.first("code_verifier");
        let client_id = query_params.first("client_id");
        let audience = query_params.first("audience");

        if let (Some(code_verifier), Some(client_id), Some(audience)) = (code_verifier, client_id, audience) {
            return Some(
                Self::builder()
                    .code_verifier(code_verifier)
                    .client_id(client_id)
                    .audience(audience)
                    .build(),
            );
        }
        None
    }
}
