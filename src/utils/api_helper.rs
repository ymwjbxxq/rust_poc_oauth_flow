use http::header::HeaderName;
use lambda_http::{Body, Request, Response, aws_lambda_events::query_map::QueryMap};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use serde_urlencoded;
use std::str;
pub struct ApiHelper;

impl ApiHelper {
    pub fn response(api_response: ApiResponse) -> Response<String> {
        let mut res = Response::builder().status(api_response.status_code.to_value());

        if let Some(headers) = api_response.headers {
            headers.iter().for_each(|(key, value)| {
                let headers = res.headers_mut().unwrap();
                headers.insert(key, value.parse().unwrap());
            });
        } else {
            let headers = res.headers_mut().unwrap();
            headers.insert(
                http::header::CONTENT_TYPE,
                String::from("application/json").parse().unwrap(),
            );
        }

        res.body(api_response.body.unwrap_or_default()).unwrap()
    }

    pub fn build_url_from_hashmap(url: String, query_string: HashMap<&str, &str>) -> String {
        let encoded = serde_urlencoded::to_string(query_string).unwrap();
        return format!("{url}?{encoded}");
    }

    pub fn build_url_from_querystring(url: String, query_string: QueryMap) -> String {
        let mut query_string_dic = HashMap::new();
        query_string.iter().for_each(|(key, value)| {
            query_string_dic.insert(key, value);
        });

        ApiHelper::build_url_from_hashmap(url, query_string_dic)
    }
}

pub struct ApiResponse {
    pub status_code: HttpStatusCode,
    pub body: Option<String>,
    pub headers: Option<HashMap<HeaderName, String>>,
}

#[derive(Copy, Clone, Debug)]
pub enum HttpStatusCode {
    Success = 200,
    Created = 201,
    Found = 302,
    Unauthorized = 401,
    NotFound = 404,
    InternalServerError = 500,
}

pub trait IntExt {
    fn to_value(&self) -> u16;
}

impl IntExt for HttpStatusCode {
    #[inline]
    fn to_value(&self) -> u16 {
        *self as u16
    }
}
