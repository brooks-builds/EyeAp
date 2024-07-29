use yew::AttrValue;
use yewdux::Store;

use crate::api::GetUserInfoResponse;

#[derive(Default, Clone, PartialEq, Store)]
pub struct MainStore {
    pub user: Option<User>,
}

#[derive(Default, Clone, PartialEq)]
pub struct User {
    pub email: AttrValue,
    pub username: AttrValue,
}

impl From<GetUserInfoResponse> for User {
    fn from(value: GetUserInfoResponse) -> Self {
        Self {
            email: value.email.into(),
            username: value.preferred_username.into(),
        }
    }
}
