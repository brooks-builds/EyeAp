use yew::prelude::*;

#[function_component]
pub fn LoginPage() -> Html {
    html! {
        <div>
            <h1>{"Login"}</h1>
            <a href="http://localhost:9011/app/login/792f4a26-2368-4566-b0b7-6935f338cf1b?redirect_uri=http://localhost:8081/auth/login&state=oqirntegjriplgjwy48ghdtnsd&scope=openid%20email%20profile">{"Login with fusion auth"}</a>
        </div>
    }
}
