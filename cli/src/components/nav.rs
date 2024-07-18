use anathema::{
    component::*,
    state::{List, State, Value},
};

use crate::routes::Route;

pub struct Nav {
    pub router_component_id: ComponentId<Route>,
}

impl Component for Nav {
    type State = NavState;

    type Message = ();

    fn on_mouse(
        &mut self,
        mouse: anathema::component::MouseEvent,
        state: &mut Self::State,
        mut elements: anathema::widgets::Elements<'_, '_>,
        context: anathema::prelude::Context<'_>,
    ) {
        if mouse.lsb_up() {
            let query = elements.query().at_position(mouse.pos());

            query.each(|_el, attr| {
                let Some(value) = attr.value() else {
                    return;
                };

                value.str_for_each(|route_name| {
                    let new_route = Route::new_from_name(route_name);
                    state.current.set(new_route.clone());
                    context
                        .emitter
                        .emit(self.router_component_id, new_route)
                        .unwrap();
                })
            })
        }
    }
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
