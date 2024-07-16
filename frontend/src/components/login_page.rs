use yew::prelude::*;

#[function_component]
pub fn LoginPage() -> Html {
    html! {
        <div>
            <h1>{"Login"}</h1>
            <a href="http://localhost:9011/oauth2/authorize?client_id=aad62697-60d9-4a67-9e1e-28f6acff23e5&response_type=code&redirect_uri=http%3A%2F%2Flocalhost%3A8081%2Fauth%2Flogin">{"Login with fusion auth"}</a>
        </div>
    }
}
