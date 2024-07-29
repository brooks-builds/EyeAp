use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

#[function_component]
pub fn AppNav() -> Html {
    html! {
        <ul>
            <li>
                <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>
            </li>
            <li>
                <Link<Route> to={Route::Login}>{"Login"}</Link<Route>>
            </li>
            <li>
                <Link<Route> to={Route::Profile}>{"Profile"}</Link<Route>>
            </li>
        </ul>
    }
}
