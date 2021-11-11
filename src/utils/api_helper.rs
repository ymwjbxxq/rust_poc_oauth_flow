use serde::{Deserialize, Serialize};
use lambda_http::StrMap;
use http::header::HeaderName;
use lambda_http::{Request, Response, Body};
use std::collections::HashMap;

use std::str;
use serde_urlencoded;
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
      headers.insert(http::header::CONTENT_TYPE, String::from("application/json").parse().unwrap());
    }

    return res.body(api_response.body.unwrap_or_default()).unwrap();
  }

  pub fn build_url_from_hashmap(url: String, query_string: HashMap<&str, &str>) -> String {
    let encoded = serde_urlencoded::to_string(&query_string).unwrap();
    return format!("{}?{}", url, encoded);
  }

  pub fn build_url_from_querystring(url: String, query_string: StrMap) -> String {
    let mut query_string_dic = HashMap::new();
    query_string.iter().for_each(|(key, value)| {
        query_string_dic.insert(key, value);
      });

    return ApiHelper::build_url_from_hashmap(url, query_string_dic);
  }
}

pub struct ApiResponse {
    pub status_code: HttpStatusCode,
    pub body: Option<String>,
    pub headers: Option<HashMap<HeaderName, String>>
}

#[derive(Copy, Clone, Debug)]
pub enum HttpStatusCode {
    Success = 200,
    Created = 201,
    Found = 302,
    Unauthorized = 401,
    NotFound = 404,
    InternalServerError = 500
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

pub trait BodyExt{
  fn get_from_body<T>(&self) -> Result<Option<T>, serde_json::Error>
                                  where T: Serialize + for<'de> Deserialize<'de>+ std::fmt::Debug ;
}

impl BodyExt for Request {
 fn get_from_body<T>(&self) -> Result<Option<T>, serde_json::Error>
                                  where T: Serialize + for<'de> Deserialize<'de> + std::fmt::Debug {

  let body_res: Result<T, serde_json::Error> = match self.body() {
    Body::Text(body) => serde_json::from_str(body),
    Body::Binary(body) => {
      let result = str::from_utf8(&body).unwrap(); //"email=a%40a.it&password=password&remember=on"
      let my_object = serde_urlencoded::from_str(result).unwrap();
      return Ok(Some(my_object));
    },
    _ => {
      log::info!("Request body is not a JSON POST");
      return Ok(None);
    }
  };
  let my_object = match body_res {
    Ok(my_object) => my_object,
    Err(err) => {
      log::info!("Failed to parse the object from request body: {}", err);
      return Ok(None);
    },
  };
  return Ok(Some(my_object));
 }
}