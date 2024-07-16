mod components;
mod router;

use crate::router::{switch, Route};
use components::nav::AppNav;
use openidconnect::{core::CoreProviderMetadata, reqwest::async_http_client, IssuerUrl};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
fn App() -> Html {
    spawn_local(async {
        let provider_metadata = CoreProviderMetadata::discover_async(
            IssuerUrl::new("http://localhost:9011/api/identity-provider".to_string()).unwrap(),
            async_http_client,
        )
        .await
        .unwrap();
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
