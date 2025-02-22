use anathema::{
    component::Component,
    state::{State, Value},
};

pub type RouteNames = [&'static str; 3];

#[derive(Copy, Clone)]
pub enum Route {
    Home,
    Login,
    Secret,
}

impl Route {
    pub fn names() -> RouteNames {
        ["Home", "Login", "Secret"]
    }

    pub fn new_from_name(name: &str) -> Self {
        match name {
            "Home" => Self::Home,
            "Login" => Self::Login,
            "Secret" => Self::Secret,
            _ => Self::Home,
        }
    }
}

impl State for Route {
    fn to_common(&self) -> Option<anathema::state::CommonVal<'_>> {
        let name = match self {
            Route::Home => "Home",
            Route::Login => "Login",
            Route::Secret => "Secret",
        };

        Some(anathema::state::CommonVal::Str(name))
    }
}

pub struct Routes;

impl Component for Routes {
    type State = RouteState;

    type Message = Route;

    fn message(
        &mut self,
        message: Self::Message,
        state: &mut Self::State,
        mut _elements: anathema::widgets::Elements<'_, '_>,
        _context: anathema::prelude::Context<'_>,
    ) {
        state.current.set(message);
    }
}

#[derive(State)]
pub struct RouteState {
    pub current: Value<Route>,
}
