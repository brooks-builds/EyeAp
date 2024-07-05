use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{home::Home, login_page::LoginPage};

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Login => html! { <LoginPage /> },
    }
}
