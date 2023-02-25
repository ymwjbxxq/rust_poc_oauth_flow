use http::{HeaderMap, HeaderValue, StatusCode};
use lambda_http::{aws_lambda_events::query_map::QueryMap, Response};
use serde_urlencoded;
use std::collections::HashMap;
use std::str;

pub enum ApiResponseType {
    Ok(String, ContentType, IsCors),
    Found(String, IsCors),
    FoundWithCustomHeaders(String, IsCors, HeaderMap),
    NotFound(String, IsCors),
    Forbidden(String, IsCors),
    NoContent(String, IsCors),
    Conflict(String, IsCors),
    BadRequest(String, IsCors),
    Created(String, IsCors),
}

pub enum IsCors {
    Yes,
    No,
}

impl IsCors {
    fn value(&self) -> bool {
        match *self {
            IsCors::Yes => true,
            _ => false,
        }
    }
}

pub enum ContentType {
    Html,
    Json,
}

impl ContentType {
    fn value(&self) -> String {
        match *self {
            ContentType::Html => "text/html".to_owned(),
            _ => "application/json".to_owned(),
        }
    }
}

impl ApiResponseType {
    pub fn build_url_from_hashmap(url: String, query_string: HashMap<&str, &str>) -> String {
        let encoded = serde_urlencoded::to_string(query_string).unwrap();
        return format!("{url}?{encoded}");
    }

    pub fn build_url_from_querystring(url: String, query_string: QueryMap) -> String {
        let mut query_string_dic = HashMap::new();
        query_string.iter().for_each(|(key, value)| {
            query_string_dic.insert(key, value);
        });

        Self::build_url_from_hashmap(url, query_string_dic)
    }

    pub fn to_response(&self) -> Response<String> {
        match self {
            ApiResponseType::Ok(body, content_type, is_cors) => {
                let mut response = Self::build_response(body, StatusCode::OK, is_cors);
                let headers = response.headers_mut();
                headers.insert(
                    http::header::CONTENT_TYPE,
                    HeaderValue::from_str(&content_type.value()).unwrap(),
                );
                response
            }
            ApiResponseType::Created(body, is_cors) => {
                Self::build_response(body, StatusCode::CREATED, is_cors)
            }
            ApiResponseType::NotFound(body, is_cors) => {
                Self::build_response(body, StatusCode::NOT_FOUND, is_cors)
            }
            ApiResponseType::Forbidden(body, is_cors) => {
                Self::build_response(body, StatusCode::FORBIDDEN, is_cors)
            }
            ApiResponseType::NoContent(body, is_cors) => {
                Self::build_response(body, StatusCode::NO_CONTENT, is_cors)
            }
            ApiResponseType::Conflict(body, is_cors) => {
                Self::build_response(body, StatusCode::CONFLICT, is_cors)
            }
            ApiResponseType::BadRequest(body, is_cors) => {
                Self::build_response(body, StatusCode::BAD_REQUEST, is_cors)
            }
            ApiResponseType::Found(target, is_cors) => {
                let mut response = Self::build_response("", StatusCode::FOUND, is_cors);
                let headers = response.headers_mut();
                headers.insert(
                    http::header::LOCATION,
                    HeaderValue::from_str(target).unwrap(),
                );
                response
            }
            ApiResponseType::FoundWithCustomHeaders(target, is_cors, custom_headers) => {
                let mut response = Self::build_response("", StatusCode::FOUND, is_cors);
                let headers = response.headers_mut();
                headers.insert(
                    http::header::LOCATION,
                    HeaderValue::from_str(target).unwrap(),
                );
                headers.extend(custom_headers.clone());
                response
            }
        }
    }

    fn build_response(body: &str, status: StatusCode, is_cors: &IsCors) -> Response<String> {
        let mut response = Response::new(body.to_owned());
        *response.status_mut() = status;
        let headers = response.headers_mut();
        Self::headers(is_cors, headers);
        response
    }

    fn headers(is_cors: &IsCors, headers: &mut HeaderMap) {
        headers.insert(
            http::header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        if is_cors.value() {
            headers.insert(
                http::header::ACCESS_CONTROL_ALLOW_ORIGIN,
                HeaderValue::from_static("*"),
            );
            headers.insert(
                http::header::ACCESS_CONTROL_ALLOW_HEADERS,
                HeaderValue::from_static("Content-Type"),
            );
            headers.insert(
                http::header::ACCESS_CONTROL_ALLOW_METHODS,
                HeaderValue::from_static("GET, POST, OPTIONS, PATCH, PUT, DELETE"),
            );
            headers.insert(
                http::header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
                HeaderValue::from_static("true"),
            );
        }
    }
}
