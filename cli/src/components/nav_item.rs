use anathema::{
    component::Component,
    state::{State, Value},
};

use crate::routes::Route;

pub struct NavItem;

impl Component for NavItem {
    type State = NavItemState;

    type Message = ();
}

#[derive(State)]
pub struct NavItemState {
    pub name: Value<Route>,
}
