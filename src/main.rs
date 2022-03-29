#![allow(dead_code)]
use log::Level;
use sycamore::prelude::*;

mod components;
mod exercises;
use components::App;

fn main() {
    console_log::init_with_level(Level::Debug).unwrap();
    sycamore::render(|ctx| {
        view! { ctx,
            App()
        }
    });
}
