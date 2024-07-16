use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{auth_login_page::AuthLogin, home::Home, login_page::LoginPage};

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/auth/login")]
    AuthLogin,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Login => html! { <LoginPage /> },
        Route::AuthLogin => html! { <AuthLogin /> },
    }
}
