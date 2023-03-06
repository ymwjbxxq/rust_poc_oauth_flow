use crate::error::ApplicationError;
use aws_lambda_events::apigw::{
    ApiGatewayCustomAuthorizerPolicy, ApiGatewayCustomAuthorizerResponse, IamPolicyStatement,
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use typed_builder::TypedBuilder as Builder;

#[derive(Debug, Serialize, Deserialize, Builder)]
pub struct Claims {
    #[builder(setter(into))]
    pub iss: String,  // "https://my-domain.authservice.com/"

    #[builder(setter(into))]
    pub sub: String, //"authservice|userId"

    #[builder(setter(into))]
    pub azp: String, //my_client_id

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
        )
        .ok();

        Ok(token)
    }

    pub fn to_response(
        &self,
        effect: String,
        principal: &str,
        method_arn: String,
    ) -> ApiGatewayCustomAuthorizerResponse {
        let stmt = IamPolicyStatement {
            action: vec!["execute-api:Invoke".to_string()],
            resource: vec![method_arn],
            effect: Some(effect),
        };
        let policy = ApiGatewayCustomAuthorizerPolicy {
            version: Some("2012-10-17".to_string()),
            statement: vec![stmt],
        };
        ApiGatewayCustomAuthorizerResponse {
            principal_id: Some(principal.to_owned()),
            policy_document: policy,
            context: json!({ "email": &principal.to_string() }),
            usage_identifier_key: None,
        }
    }

    pub async fn validate_token(
        &self,
        raw_token: &str,
    ) -> Result<Option<Claims>, ApplicationError> {
        if let Some(token) = self.get_token(raw_token) {
          let token_data = decode::<Value>(
                &token,
                &DecodingKey::from_secret(self.private_key.as_bytes()),
                &Validation::new(Algorithm::HS256),
            )
            .ok();
            if let Some(token_data) = token_data {
                let claims: Claims = serde_json::from_value(token_data.claims)?;
                return Ok(Some(claims));
            }
        }
        Ok(None)
    }
}
