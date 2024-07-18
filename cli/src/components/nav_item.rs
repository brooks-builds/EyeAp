use anathema::{
    component::Component,
    state::{State, Value},
    widgets::components::Context,
};

use crate::routes::Route;

pub struct NavItem;

impl Component for NavItem {
    type State = NavItemState;

    type Message = ();

    fn on_mouse(
        &mut self,
        mouse: anathema::component::MouseEvent,
        state: &mut Self::State,
        mut elements: anathema::widgets::Elements<'_, '_>,
        _context: Context,
    ) {
        if mouse.lsb_up() {
            let el = elements.query().at_position(mouse.pos());
            el.first(|_element, attributes| {
                let a = attributes.value();
                println!("{a:?}");
            });
            state.clicked.set(true);
        }
    }
}

#[derive(State)]
pub struct NavItemState {
    pub name: Value<Route>,
    pub clicked: Value<bool>,
}
