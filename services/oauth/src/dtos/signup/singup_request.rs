use lambda_http::{Body, RequestExt};
use serde::{Deserialize, Deserializer, Serialize};
use typed_builder::TypedBuilder as Builder;

#[derive(Debug, Builder, Deserialize, Serialize)]
pub struct SignUpRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option))]
    pub client_id: Option<String>,

    #[builder(setter(into))]
    pub email: String,

    #[builder(setter(into))]
    pub password: String,

    #[builder(setter(into))]
    pub family_name: String,

    #[builder(setter(into))]
    pub given_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "deserialize_bool_from_string")]
    #[serde(default)]
    #[builder(setter(strip_option))]
    pub is_consent: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "deserialize_bool_from_string")]
    #[serde(default)]
    #[builder(setter(strip_option))]
    pub is_optin: Option<bool>,
}

fn deserialize_bool_from_string<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    match s {
        "true" => Ok(Some(true)),
        "false" => Ok(Some(false)),
        _ => Ok(None),
    }
}

impl SignUpRequest {
    pub fn validate(event: &http::Request<Body>) -> Option<SignUpRequest> {
        let user = event.payload::<SignUpRequest>().ok().and_then(|user| user);
        if let Some(user) = user {
            let query_params = event.query_string_parameters();
            let client_id = query_params.first("client_id");
            if let Some(client_id) = client_id {
                return Some(
                    Self::builder()
                        .password(user.password)
                        .email(user.email)
                        .client_id(client_id.to_lowercase())
                        .family_name(user.family_name)
                        .given_name(user.given_name)
                        .is_consent(user.is_consent.unwrap_or_default())
                        .is_optin(user.is_optin.unwrap_or_default())
                        .build(),
                );
            }
        }
        None
    }
}
