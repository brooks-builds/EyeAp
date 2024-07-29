use yew::prelude::*;

use crate::api::get_user_info;

#[function_component]
pub fn AuthLogin() -> Html {
    use_effect(|| {
        wasm_bindgen_futures::spawn_local(async {
            get_user_info().await;
        });

        || {}
    });
    html! {
        <div>
            <h1>{"Auth Login"}</h1>
        </div>
    }
}
