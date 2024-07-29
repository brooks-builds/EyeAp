mod components;
mod pages;
mod routes;

use std::fs::read_to_string;

use anathema::{backend::tui::TuiBackend, runtime::Runtime, state::Value, templates::Document};
use components::{
    input::{IndexState, Input},
    nav::{Nav, NavState},
};
use eyre::{Context, Result};
use pages::{home::Home, login::Login};
use routes::{RouteState, Routes};

pub fn run() -> Result<()> {
    let index_template = read_to_string("templates/index.aml").context("loading index template")?;

    let document = Document::new(index_template);
    let backend = TuiBackend::builder()
        // .enable_alt_screen()
        .enable_raw_mode()
        .enable_mouse()
        .hide_cursor()
        .finish()
        .context("creating backend")?;
    let mut runtime_builder = Runtime::builder(document, backend);

    let router = runtime_builder
        .register_component(
            "router",
            "templates/router.aml",
            Routes,
            RouteState {
                current: Value::new(routes::Route::Home),
            },
        )
        .unwrap();
    let _nav_component = runtime_builder.register_component(
        "nav",
        "templates/components/nav.aml",
        Nav {
            router_component_id: router,
        },
        NavState::new(routes::Route::Home),
    );
    let _home_page =
        runtime_builder.register_component("home", "templates/pages/home.aml", Home, ());
    let _login_page =
        runtime_builder.register_component("login", "templates/pages/login.aml", Login, ());
    let _input = runtime_builder.register_prototype(
        "input",
        "templates/components/input.aml",
        || Input,
        IndexState::new,
    );

    let mut runtime = runtime_builder.finish().context("creating runtime")?;

    runtime.run();
    Ok(())
}
