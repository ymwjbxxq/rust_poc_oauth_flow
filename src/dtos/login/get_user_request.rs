use lambda_http::{Body, RequestExt};
use typed_builder::TypedBuilder as Builder;
use crate::models::user::User;

#[derive(Debug, Builder)]
pub struct GetUserRequest {
    #[builder(default, setter(into))]
    pub email: String,

    #[builder(default, setter(into))]
    pub password: String,

    #[builder(default, setter(into))]
    pub client_id: String,

    #[builder(default, setter(into))]
    pub host: String,
}

impl GetUserRequest {
    pub fn validate(event: &http::Request<Body>) -> Option<GetUserRequest> {
        let user = event.payload::<User>().ok().and_then(|user| user);

        if let Some(user) = user {
            let query_params = event.query_string_parameters();
            let client_id = query_params.first("client_id");
            let host = event.headers().get("Host");

            if let (Some(client_id), Some(host)) = (client_id, host) {
                return Some(
                    Self::builder()
                        .email(user.email.unwrap())
                        .password(user.password.unwrap())
                        .client_id(client_id)
                        .host(host.to_str().unwrap())
                        .build(),
                );
            }
        }
        None
    }
}
