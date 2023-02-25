use typed_builder::TypedBuilder as Builder;
use serde::{Deserialize, Serialize};
use lambda_http::{Body, RequestExt};

#[derive(Builder, Deserialize, Serialize)]
pub struct SignUpRequest {
    #[builder(setter(into))]
    pub client_id: String,

    #[builder(setter(into))]
    pub email: String,

    #[builder(setter(into))]
    pub password: String,

    #[builder(setter(into))]
    pub family_name: String,

    #[builder(setter(into))]
    pub given_name: String,


    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option))]
    pub is_consent: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option))]
    pub is_optin: Option<bool>,
}

impl SignUpRequest {
    pub fn validate(event: &http::Request<Body>) -> Option<SignUpRequest> {
        let user = event.payload::<SignUpRequest>().ok().and_then(|user| user);
        if let Some(user) = user {
            let query_params = event.query_string_parameters();
            let client_id = query_params.first("client_id");

            if let Some(client_id) = client_id
            {
                return Some(
                    Self::builder()
                        .password(user.password)
                        .email(user.email)
                        .client_id(client_id)
                        .family_name(user.family_name)
                        .given_name(user.given_name)
                        .is_consent(user.is_consent.is_some())
                        .is_optin(user.is_optin.is_some())
                        .build(),
                );
            }
        }
        None
    }
}