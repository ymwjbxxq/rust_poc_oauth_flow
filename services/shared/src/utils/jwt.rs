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
    pub iss: String, // "https://my-domain.authservice.com/"

    #[builder(setter(into))]
    pub sub: String, //"authservice|userId"

    #[builder(setter(into))]
    pub azp: String, //my_client_id

    #[builder(setter(into))]
    pub aud: String, //my_app

    #[builder(setter(into))]
    pub exp: i64,
}

#[derive(Debug, Clone, Default)]
pub struct Jwt {}

impl Jwt {
    fn get_token(raw_token: &str) -> Option<String> {
        let token = raw_token.strip_prefix("Bearer ");

        token.map(str::to_string)
    }

    pub fn encode(claims: &Claims, private_key: &str) -> anyhow::Result<Option<String>> {
        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::new(Algorithm::RS256),
            claims,
            &jsonwebtoken::EncodingKey::from_rsa_pem(private_key.as_bytes())
                .expect("invalid private key"),
        )
        .ok();

        Ok(token)
    }

    pub fn to_response(
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

    pub async fn validate_token(raw_token: &str) -> anyhow::Result<Option<Claims>> {
        if let Some(token) = Jwt::get_token(raw_token) {
            let public_key = Jwt::get_public_key().await?;
            let token_data = decode::<Value>(
                &token,
                &DecodingKey::from_rsa_pem(public_key.as_bytes()).expect("invalid public key"),
                &Validation::new(Algorithm::RS256),
            )
            .ok();
            if let Some(token_data) = token_data {
                let claims: Claims = serde_json::from_value(token_data.claims)?;
                return Ok(Some(claims));
            }
        }
        Ok(None)
    }

    async fn get_public_key() -> anyhow::Result<String> {
        let json_key_set_url = std::env::var("JSKS_URI").expect("JSKS_URI must be set");
        let res = reqwest::Client::new().get(json_key_set_url).send().await?;
        let jwks = res.json::<JwtKeys>().await?;

        Ok(jwks.public_key.replace('\n', ""))
    }
}

#[derive(Deserialize)]
pub struct JwtKeys {
    // pub keys: Vec<JwtKey>,
    pub public_key: String,
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct JwtKey {
//     pub e: String,
//     pub kty: String,
//     pub alg: Option<String>,
//     pub n: String,
//     pub kid: String,
// }

// impl Clone for JwtKey {
//     fn clone(&self) -> Self {
//         JwtKey {
//             e: self.e.clone(),
//             kty: self.kty.clone(),
//             alg: self.alg.clone(),
//             n: self.n.clone(),
//             kid: self.kid.clone(),
//         }
//     }
// }
