#![feature(async_await)]

mod renderer;
mod assets;

use cfg_if::cfg_if;
use log::{info, Level};
use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement};

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
pub fn run(canvas:HtmlCanvasElement, window_width: u32, window_height: u32, send_event: js_sys::Function) -> Result<JsValue, JsValue> {
    init_panic();
    init_log();

    info!("Starting renderer...");

    renderer::start(canvas, window_width, window_height, send_event)
}

