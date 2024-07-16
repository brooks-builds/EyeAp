use anathema::{
    component::Component,
    runtime::{ComponentId, Emitter},
    state::{CommonVal, List, State, Value},
};

use crate::routes::{Route, Routes};

pub struct Nav;

impl Component for Nav {
    type State = NavState;

    type Message = ();

    fn on_mouse(
        &mut self,
        mouse: anathema::component::MouseEvent,
        state: &mut Self::State,
        mut elements: anathema::widgets::Elements<'_, '_>,
        viewport: anathema::prelude::Viewport,
    ) {
        if mouse.lsb_up() {
            let query = elements.query().at_position(mouse.pos());

            query.each(|el, attr| {
                let Some(value) = attr.value() else {
                    return;
                };

                value.str_for_each(|route_name| {
                    let new_route = Route::new_from_name(route_name);
                    state.current.set(new_route);
                })
            })
        }
    }
}

#[derive(State)]
pub struct NavState {
    routes: Value<List<String>>,
    current: Value<Route>,
    routes_emitter: Value<EmitterWrapper>,
}

impl NavState {
    pub fn new(route: Route, routes_emitter: Emitter, routes_id: ComponentId<Route>) -> Self {
        let routes = List::from_iter(Route::names().map(ToString::to_string)).into();
        let current = Value::new(route);
        let routes_emitter = Value::new(EmitterWrapper {
            emitter: routes_emitter,
            component_id: routes_id,
        });

        Self {
            routes,
            current,
            routes_emitter,
        }
    }
}

pub struct EmitterWrapper {
    pub emitter: Emitter,
    pub component_id: ComponentId<Route>,
}

impl State for EmitterWrapper {
    fn to_common(&self) -> Option<anathema::state::CommonVal<'_>> {
        Some(CommonVal::Str("this emitter should never be displayed"))
    }
}
