use lambda_http::{Body, RequestExt};
use typed_builder::TypedBuilder as Builder;

#[derive(Debug, Builder)]
pub struct UpdateConsentRequest {
    #[builder(default, setter(into))]
    pub is_optin: String,

    #[builder(default, setter(into))]
    pub user: String,

    #[builder(default, setter(into))]
    pub client_id: String,

    #[builder(default, setter(into))]
    pub host: String,
}

impl UpdateConsentRequest {
    pub fn validate(event: &http::Request<Body>) -> Option<UpdateConsentRequest> {
        // if let Some(cookie) = cookie {
        //     let is_optin = cookie.get("is_optin");
        //     let user = cookie.get("user");
        //     let query_params = event.query_string_parameters();
        //     let client_id = query_params.first("client_id");
        //     let host = event.headers().get("Host");

        //     if let (Some(is_optin), Some(user), Some(client_id), Some(host)) =
        //         (is_optin, user, client_id, host)
        //     {
        //         return Some(
        //             Self::builder()
        //                 .is_optin(is_optin)
        //                 .user(user)
        //                 .client_id(client_id)
        //                 .host(host.to_str().unwrap())
        //                 .build(),
        //         );
        //     }
        // }
        None
    }
}
