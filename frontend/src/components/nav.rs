use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::use_store;

use crate::{router::Route, store::MainStore};

#[function_component]
pub fn AppNav() -> Html {
    let (store, _dispatch) = use_store::<MainStore>();

    html! {
        <ul>
            <li>
                <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>
            </li>
            <li>
                <Link<Route> to={Route::Profile}>{"Profile"}</Link<Route>>
            </li>
            if store.logged_in() {
                <li>
                    <a href="http://localhost:9011/app/logout/792f4a26-2368-4566-b0b7-6935f338cf1b?redirect_uri=http://localhost:8081">{"Logout"}</a>
                </li>
            }
            <li>
                <Link<Route> to={Route::Login}>{"Login"}</Link<Route>>
            </li>
        </ul>
    }
}
