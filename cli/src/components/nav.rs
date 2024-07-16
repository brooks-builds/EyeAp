use anathema::{
    component::Component,
    state::{List, State, Value},
};

use crate::routes::Route;

pub struct Nav;

impl Component for Nav {
    type State = NavState;

    type Message = ();
}

#[derive(State)]
pub struct NavState {
    routes: Value<List<String>>,
    current: Value<Route>,
}

impl NavState {
    pub fn new(route: Route) -> Self {
        let routes = List::from_iter(Route::names().map(ToString::to_string)).into();
        let current = Value::new(route);

        Self { routes, current }
    }
}
