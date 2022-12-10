use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

mod body;
mod index;
mod header;
mod lpanel;
mod rpanel;
mod content;
mod snippet;
mod shatters;

use index::Index;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Tab {
    Home,
    Shatters,
    Bibliothek,
    AboutMe,
}

pub enum Msg {
    Routing(Tab),
}

pub const HOME: &str = "#/";
pub const SHATTERS: &str = "#/shatters";
pub const BIBLIOTHEK: &str = "#/bibliothek";
pub const ABOUTME: &str = "#/about";

pub fn translate_tab(hash: &str) -> Option<Tab> {
    match hash {
        HOME => Some(Tab::Home),
        SHATTERS => Some(Tab::Shatters),
        BIBLIOTHEK => Some(Tab::Bibliothek),
        ABOUTME => Some(Tab::AboutMe),
        _ => None
    }
}

#[derive(PartialEq, Eq, Properties)]
pub struct Props {
    #[prop_or(Tab::Home)]
    pub tab: Tab,
}

pub fn route(cb: Callback<Tab>) {
    let window = web_sys::window().unwrap();
    let dispatch = move || {
        let hash = window
        .location()
        .hash()
        .unwrap();

        if let Some(tab) = translate_tab(hash.as_str()) {
            cb.emit(tab);
        }
    };
    dispatch();

    let cb = Closure::<dyn Fn()>::new(move || { dispatch(); });
    web_sys::window()
    .unwrap()
    .add_event_listener_with_callback(
            "hashchange",
            cb.as_ref().unchecked_ref()
    ).unwrap();
    cb.forget();
}

fn main() {
    yew::start_app::<Index>();
}
