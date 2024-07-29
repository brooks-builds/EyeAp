use gloo::net::http::Request;
use serde::{Deserialize, Serialize};

pub async fn get_user_info() -> Option<GetUserInfoResponse> {
    let url = "http://localhost:9011/app/me/";

    let response = Request::get(url)
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .unwrap();

    if !response.ok() {
        return None;
    }

    response
        .json()
        .await
        .expect("converting user info to rust struct")
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetUserInfoResponse {
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub given_name: Option<String>,
    pub preferred_username: Option<String>,
}
