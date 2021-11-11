use aws_sdk_dynamodb::SdkError;
use aws_sdk_dynamodb::model::AttributeValue;
use std::fmt;
use std::error;

#[derive(Debug)]
pub enum ApplicationError {
    InitError(&'static str),
    ClientError(&'static str),
    InternalError(&'static str),
    SdkError(String),
}

impl std::error::Error for ApplicationError {}

impl fmt::Display for ApplicationError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      ApplicationError::InitError(msg) => write!(f, "InitError: {}", msg),
      ApplicationError::ClientError(msg) => write!(f, "ClientError: {}", msg),
      ApplicationError::InternalError(msg) => write!(f, "InternalError: {}", msg),
      ApplicationError::SdkError(err) => write!(f, "SdkError: {}", err),
    }
  }
}

impl From<std::str::Utf8Error> for ApplicationError {
  fn from(_: std::str::Utf8Error) -> ApplicationError {
    ApplicationError::InternalError("Converts a slice of bytes to a string slice failed")
  }
}

impl From<base64::DecodeError> for ApplicationError {
  fn from(_: base64::DecodeError) -> ApplicationError {
    ApplicationError::InternalError("Decode base64 to string failed")
  }
}

impl From<&AttributeValue> for ApplicationError {
  fn from(_: &AttributeValue) -> ApplicationError {
    ApplicationError::InternalError("DynamoDB Invalid value type")
  }
}

impl<E> From<SdkError<E>> for ApplicationError
where
  E: error::Error,
{
  fn from(value: SdkError<E>) -> ApplicationError {
    ApplicationError::SdkError(format!("sdk error {}", value))
  }
}