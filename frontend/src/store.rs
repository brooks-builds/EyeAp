use yew::AttrValue;
use yewdux::Store;

use crate::api::GetUserInfoResponse;

#[derive(Default, Clone, PartialEq, Store)]
pub struct MainStore {
    pub user: Option<User>,
}

impl MainStore {
    pub fn logged_in(&self) -> bool {
        self.user.is_some()
    }
}

#[derive(Default, Clone, PartialEq)]
pub struct User {
    pub email: AttrValue,
    pub username: AttrValue,
}

impl From<GetUserInfoResponse> for User {
    fn from(value: GetUserInfoResponse) -> Self {
        Self {
            email: value.email.unwrap_or_default().into(),
            username: value.preferred_username.unwrap_or_default().into(),
        }
    }
}
