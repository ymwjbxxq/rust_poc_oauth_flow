use serde::{ser::SerializeStruct, Serialize};
use serde_json::Value;
use serde::{Deserialize};
use lambda_runtime::{handler_fn, Error, Context};
use log::LevelFilter;
use simple_logger::SimpleLogger;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, DecodingKey, Validation};

#[tokio::main]
async fn main() -> Result<(), Error> {
  // Initialize service
  SimpleLogger::new()
    .with_level(LevelFilter::Info)
    .init()
    .unwrap();

  lambda_runtime::run(handler_fn(|event: Value, ctx: Context| {
        execute(event, ctx)
    })) 
    .await?;

  Ok(())
}

pub async fn execute(event: Value, _ctx: Context) -> Result<AuthorizerResponse, Error> {
  let request: AuthRequest = serde_json::from_value(event)?;
  log::info!("EVENT {:?}", request);
  if let Some(authorization) = request.headers.authorization {
    let token = decode::<Claims>(&authorization, &DecodingKey::from_secret("privateKey".as_bytes()), &Validation::default());
    return Ok(match token {
      Ok(_token) => {
        AuthorizerResponse {
          isAuthorized: true,
          context: ResponseContext {
            message: None,
            AuthInfo: String::from("true-users"),
          }
        }
      },
      Err(err) => match *err.kind() {
          ErrorKind::InvalidToken => {
            AuthorizerResponse {
              isAuthorized: false,
              context: ResponseContext {
                message: Some(String::from("Invalid token")),
                AuthInfo: String::from("true-users"),
              }
            }
          },
          ErrorKind::InvalidIssuer => {
            AuthorizerResponse {
              isAuthorized: false,
              context: ResponseContext {
                message: Some(String::from("Invalid issuer")),
                AuthInfo: String::from("true-users"),
              }
            }
          },
          _ => panic!("Some other errors"),
      },
    });
  } 

  Ok(AuthorizerResponse {
    isAuthorized: false,
    context: ResponseContext {
      message: Some(String::from("Authorization header is missing")),
      AuthInfo: String::from("true-users"),
    }
  })
}

#[derive(Deserialize, Debug, Default)]
pub struct AuthRequest {
  pub headers: RequestHeader,
}

#[derive(Deserialize, Debug, Default)]
pub struct RequestHeader {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub authorization: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
  sub: String,
  company: String,
  exp: usize,
}

#[derive(Serialize)]
pub struct AuthorizerResponse {
  pub isAuthorized: bool,
  pub context: ResponseContext,
}

#[derive(Serialize)]
pub struct ResponseContext {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
  pub AuthInfo: String,
}

/// Optimized response representation
/// https://n14n.dev/articles/2021/safe-json-representations-with-rust/
/// Developers cannot create an invalid response with this enum. However, we
/// have to create a custom serializer to properly transform it into a valid
/// JSON response, but this is an acceptable trade-off to make this library
/// safer to use.
/// 
/// If the success field was a string instead of a bool, we could use the
/// enum tag representation directly.
/// See https://serde.rs/enum-representations.html to learn more.
pub enum ConditionalFields {
    Success(AuthorizerResponse),
    Error(String),
}

impl Serialize for ConditionalFields {
    /// Custom serializer for ConditionalFields
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ConditionalFields::Success(context) => {
                let mut s = serializer.serialize_struct("ConditionalFields", 2)?;
                s.serialize_field("isAuthorized", &true)?;
                s.serialize_field("context", context)?;
                s.end()
            }
            ConditionalFields::Error(error) => {
                let mut s = serializer.serialize_struct("ConditionalFields", 2)?;
                s.serialize_field("isAuthorized", &false)?;
                s.serialize_field("context", error)?;
                s.end()
            }
        }
    }
}