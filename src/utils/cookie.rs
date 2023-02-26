use crate::error::ApplicationError;
use crate::utils::serde_helper::SerdeExt;
use cookie::Cookie;
use http::{HeaderMap, HeaderValue};
use serde_json::{Map, Value};
use std::collections::HashMap;

pub trait CookieExt {
    fn to_map(parsed_cookie: String) -> HashMap<String, String>;
    fn to_cookie_string(cookie_name: String, cookie: HashMap<String, String>) -> String;
}

impl CookieExt for Cookie<'_> {
    fn to_map(parsed_cookie: String) -> HashMap<String, String> {
        let cookie = Cookie::parse(parsed_cookie).unwrap();
        let parsed: Value = serde_json::from_str(cookie.value()).unwrap();
        let serde_dic: Map<String, Value> = parsed.as_object().unwrap().clone();
        let mut cookie_dic = HashMap::new();
        for (key, value) in serde_dic.into_iter() {
            cookie_dic.insert(key.to_owned(), value.value_to_string());
        }

        cookie_dic
    }

    fn to_cookie_string(cookie_name: String, cookie_hash_map: HashMap<String, String>) -> String {
        let cookie = Cookie::build(
            cookie_name,
            serde_json::to_string(&cookie_hash_map).unwrap(),
        )
        .path("/")
        .secure(true)
        .http_only(true)
        .finish();

        cookie.to_string()
    }
}

pub struct CookieHelper;

impl CookieHelper {
    pub fn from_http_header(
        headers: &HeaderMap<HeaderValue>,
    ) -> Result<HashMap<String, String>, ApplicationError> {
        let cookie_header = match headers.get(http::header::COOKIE) {
            Some(result) => result,
            None => {
                return Err(ApplicationError::InternalError(
                    "Cannot find cookie in the header request".to_owned(),
                ))
            }
        };

        if let Ok(cookie) = cookie_header.to_str() {
            Ok(Cookie::to_map(cookie.to_string()))
        } else {
            Err(ApplicationError::InternalError(
                "Something wrong with the cookie value".to_owned(),
            ))
        }
    }
}
