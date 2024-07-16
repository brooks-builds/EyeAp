mod components;
mod pages;
mod routes;

use std::fs::read_to_string;

use anathema::{prelude::*, state::Value};
use components::{
    nav::{Nav, NavState},
    nav_item::{NavItem, NavItemState},
};
use eyre::{Context, Result};
use pages::{home::Home, login::Login};
use routes::{RouteState, Routes};

pub fn run() -> Result<()> {
    let index_template = read_to_string("templates/index.aml").context("loading index template")?;
    let nav_template =
        read_to_string("templates/components/nav.aml").context("loading nav component template")?;
    let home_template =
        read_to_string("templates/pages/home.aml").context("loading home page template")?;
    let login_template =
        read_to_string("templates/pages/login.aml").context("loading login page template")?;
    let router_template =
        read_to_string("templates/router.aml").context("loading router template")?;
    let nav_item_template =
        read_to_string("templates/components/nav_item.aml").context("loading nav item template")?;

    let document = Document::new(index_template);
    let backend = TuiBackend::builder()
        .enable_mouse()
        .hide_cursor()
        .finish()
        .context("creating backend")?;
    let mut runtime_builder = Runtime::builder(document, backend);

    let emitter = runtime_builder.emitter();
    let router = runtime_builder.register_component(
        "router",
        router_template,
        Routes,
        RouteState {
            current: Value::new(routes::Route::Login),
        },
    );
    let _nav_component = runtime_builder.register_component(
        "nav",
        nav_template,
        Nav,
        NavState::new(routes::Route::Login, emitter, router),
    );
    let _home_page = runtime_builder.register_component("home", home_template, Home, ());
    let _login_page = runtime_builder.register_component("login", login_template, Login, ());
    let _nav_item = runtime_builder.register_component(
        "nav_item",
        nav_item_template,
        NavItem,
        NavItemState {
            name: Value::new(routes::Route::Login),
            clicked: Value::default(),
        },
    );

    let mut runtime = runtime_builder.finish().context("creating runtime")?;

    runtime.run().context("running")?;
    Ok(())
}
