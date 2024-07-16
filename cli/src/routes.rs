use anathema::{
    component::Component,
    state::{State, Value},
};

pub type RouteNames = [&'static str; 2];

pub enum Route {
    Home,
    Login,
}

impl Route {
    pub fn names() -> RouteNames {
        ["Home", "Login"]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Route::Home => "Home",
            Route::Login => "Login",
        }
    }
}

impl State for Route {
    fn to_common(&self) -> Option<anathema::state::CommonVal<'_>> {
        let name = match self {
            Route::Home => "Home",
            Route::Login => "Login",
        };

        Some(anathema::state::CommonVal::Str(name))
    }
}

pub struct Routes;

impl Component for Routes {
    type State = RouteState;

    type Message = ();
}

#[derive(State)]
pub struct RouteState {
    pub current: Value<Route>,
}
