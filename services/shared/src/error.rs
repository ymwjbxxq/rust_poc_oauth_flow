use aws_sdk_dynamodb::{self, types::SdkError};
use jsonwebtoken;
use serde_json;
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum ApplicationError {
    InitError(String),
    ClientError(String),
    InternalError(String),
    SdkError(String),
}

impl std::error::Error for ApplicationError {}

impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApplicationError::InitError(msg) => write!(f, "{msg}"),
            ApplicationError::ClientError(msg) => write!(f, "{msg}"),
            ApplicationError::InternalError(msg) => write!(f, "{msg}"),
            ApplicationError::SdkError(err) => write!(f, "{err}"),
        }
    }
}

impl From<serde_json::error::Error> for ApplicationError {
    fn from(value: serde_json::error::Error) -> ApplicationError {
        ApplicationError::ClientError(format!("Cannot convert to string {value}"))
    }
}

impl<E> From<SdkError<E>> for ApplicationError
where
    E: error::Error,
{
    fn from(value: SdkError<E>) -> ApplicationError {
        ApplicationError::SdkError(format!("{value}"))
    }
}

impl From<Box<dyn std::error::Error + Sync + std::marker::Send>> for ApplicationError {
    fn from(value: Box<dyn std::error::Error + Sync + std::marker::Send>) -> Self {
        ApplicationError::InternalError(format!("{value:?}"))
    }
}

impl From<jsonwebtoken::errors::Error> for ApplicationError {
    fn from(e: jsonwebtoken::errors::Error) -> ApplicationError {
        ApplicationError::ClientError(format!("Problem decoding the token {e}"))
    }
}

impl From<std::str::Utf8Error> for ApplicationError {
    fn from(_: std::str::Utf8Error) -> ApplicationError {
        ApplicationError::InternalError(
            "Converts a slice of bytes to a string slice failed".to_owned(),
        )
    }
}

impl From<base64::DecodeError> for ApplicationError {
    fn from(_: base64::DecodeError) -> ApplicationError {
        ApplicationError::InternalError("Decode base64 to string failed".to_owned())
    }
}

impl From<reqwest::Error> for ApplicationError {
    fn from(e: reqwest::Error) -> ApplicationError {
        if e.is_timeout() {
            return ApplicationError::ClientError(
                "TIMEOUT: The request timed out while trying to connect to the remote server"
                    .to_string(),
            );
        }

        ApplicationError::SdkError(format!("reqwest sdk error {e:?}"))
    }
}