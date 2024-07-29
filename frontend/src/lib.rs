pub mod api;
mod components;
mod router;
pub mod store;

use crate::router::{switch, Route};
use components::nav::AppNav;
use store::MainStore;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::use_store;

#[function_component]
fn App() -> Html {
    let (_store, dispatch) = use_store::<MainStore>();

    use_effect(move || {
        let dispatch = dispatch.clone();

        spawn_local(async move {
            if let Some(user) = api::get_user_info().await {
                dispatch.reduce_mut(|store| {
                    store.user = Some(user.into());
                });
            };
        });

        || {}
    });

    html! {
        <BrowserRouter>
            <h1>{"EyeAP"}</h1>
            <AppNav />
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

pub fn run() {
    yew::Renderer::<App>::new().render();
}
