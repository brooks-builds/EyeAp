use yew::prelude::*;
use yewdux::use_store;

use crate::store::MainStore;

#[function_component()]
pub fn ProfilePage() -> Html {
    let (store, _dispatch) = use_store::<MainStore>();

    html! {
        <section>
            <h2>{"Profile"}</h2>
            if store.user.is_none() {
                <p>{"Loading profile"}</p>
            } else {
                <p>{"Username: "} {store.user.clone().unwrap().username}</p>
                <p>{"email: "} {store.user.clone().unwrap().email}</p>
            }
        </section>
    }
}
