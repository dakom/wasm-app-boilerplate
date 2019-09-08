#![feature(async_await)]

mod components;
mod events;
mod state;
mod core_loop;

use cfg_if::cfg_if;
use log::{info, Level};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// enable logging only during debug builds
cfg_if! {
    if #[cfg(feature = "console_log")] {
        fn init_log() {
            use console_log;
            console_log::init_with_level(Level::Trace).expect("error initializing log");
        }
    } else {
        fn init_log() {}
    }
}

// enable panic hook only during debug builds
cfg_if! {
    if #[cfg(feature = "console_error_panic_hook")] {
        fn init_panic() {
            console_error_panic_hook::set_once();
        }
    } else {
        fn init_panic() {}
    }
}

// Called by our JS entry point to run the example.
#[wasm_bindgen]
pub fn run(on_ui_state: js_sys::Function, on_render_state:js_sys::Function, on_audio_state:js_sys::Function) -> Result<JsValue, JsValue> {
    init_panic();
    init_log();

    core_loop::start(on_ui_state, on_render_state, on_audio_state)
}

