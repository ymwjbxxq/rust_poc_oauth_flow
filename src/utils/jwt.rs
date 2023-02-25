use crate::error::ApplicationError;
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use typed_builder::TypedBuilder as Builder;

#[derive(Debug, Serialize, Deserialize, Builder)]
pub struct Claims {
    #[builder(setter(into))]
    pub sub: String,

    #[builder(setter(into))]
    pub company: String,

    #[builder(setter(into))]
    pub exp: i64,
}

#[derive(Debug, Clone, Default)]
pub struct Jwt {
    private_key: String,
}

impl Jwt {
    pub fn new(private_key: &str) -> Self {
        Jwt {
            private_key: private_key.to_owned(),
        }
    }

    fn get_token(&self, raw_token: &str) -> Option<String> {
        let token = raw_token.strip_prefix("Bearer ");

        token.map(str::to_string)
    }

    pub fn encode(&self, claims: &Claims) -> Result<Option<String>, ApplicationError> {
        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::new(Algorithm::HS256),
            claims,
            &jsonwebtoken::EncodingKey::from_secret(self.private_key.as_bytes()),
        ).ok();

        Ok(token)
    }

    // pub fn decode(&self, raw_token: &str) -> Result<Option<Claims>, ApplicationError> {
    //     if let Some(raw_token) = self.get_token(raw_token) {
    //         let token = decode::<Claims>(
    //             &raw_token,
    //             &DecodingKey::from_secret(self.private_key.as_bytes()),
    //             &Validation::default(),
    //         )
    //         .ok();
    //         if let Some(token) = token {
    //             let result = decode::<Value>(
    //                         &token,
    //                         &DecodingKey::from_rsa_components(&jwk.n, &jwk.e)?,
    //                         &validation,
    //                     );

    //             let claims: Claims = serde_json::from_value(token.claims)?;
    //             return Ok(Some(claims));
    //         }
    //     }

    //     Ok(None)
    // }
}
