use anathema::{component::Component, state::Value};

pub struct Login;

impl Component for Login {
    type State = ();

    type Message = ();

    fn on_mouse(
        &mut self,
        mouse: anathema::component::MouseEvent,
        _state: &mut Self::State,
        mut elements: anathema::widgets::Elements<'_, '_>,
        _context: anathema::prelude::Context<'_>,
    ) {
        elements
            .query()
            .at_position(mouse.pos())
            .first(|_element, attributes| {
                let button_type = attributes.contains("button_type");
                println!("clicked login button: {button_type}");
            });
    }
}
