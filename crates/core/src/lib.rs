#![feature(async_await)]

mod components;
mod systems;
mod events;
mod world;

use cfg_if::cfg_if;
use log::{info, Level};
use wasm_bindgen::prelude::*;
use std::rc::{Rc};
use std::cell::{RefCell};
use awsm_web::tick::{MainLoop, MainLoopOptions};
use crate::events::{handle_event};
use crate::world::init_world;
use crate::systems::render::{extract_render_state};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// enable logging only during debug builds
cfg_if! {
    if #[cfg(all(feature = "console_log", debug_assertions))] {
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
    if #[cfg(all(feature = "console_error_panic_hook", debug_assertions))] {
        fn init_panic() {
            console_error_panic_hook::set_once();
        }
    } else {
        fn init_panic() {}
    }
}

// Called by our JS entry point to run the example.
#[wasm_bindgen]
pub fn run(on_ui_state: js_sys::Function, on_render_state:js_sys::Function, on_audio_state:js_sys::Function, window_width: u32, window_height: u32) -> Result<JsValue, JsValue> {
    init_panic();
    init_log();

    start_loop(on_ui_state, on_render_state, on_audio_state, window_width, window_height)
}


fn start_loop(on_ui_state: js_sys::Function, on_render_state:js_sys::Function, on_audio_state:js_sys::Function, window_width: u32, window_height: u32) -> Result<JsValue, JsValue> {
    let world = Rc::new(init_world(window_width, window_height));

    //Create a function which allows JS to send us events ad-hoc
    //We will need to get a handle and forget the Closure
    //See https://stackoverflow.com/a/53219594/784519
    let _send_event = Closure::wrap(Box::new({
        let world = Rc::clone(&world);

        move |evt_type:u32, data:JsValue| {
            {
                //The actual handling of events is in this function
                match handle_event(evt_type, data, &world) {
                    Ok(_) => {},
                    Err(reason) => info!("Error: {:?}", reason)
                }
            }
        }
    }) as Box<dyn FnMut(u32, JsValue) -> ()>);

    let send_event = _send_event.as_ref().clone();
    _send_event.forget();

    //Main loop callbacks
    let begin = {
        let world = Rc::clone(&world);
        move |time, delta| {
        }
    };

    let update = {
        let world = Rc::clone(&world);
        move |delta| {
        }
    };

    let draw = {
        let world = Rc::clone(&world);
        move |interpolation| {
            let this = JsValue::NULL;

            let render_state = extract_render_state(&world, interpolation);
            let render_state = serde_wasm_bindgen::to_value(&render_state).unwrap();
            on_render_state.call1(&this, &render_state);

            /* 
            let ui_state = extract_ui_state_js(&state, interpolation);
            on_ui_state.call1(&this, &ui_state);
            
            let render_state = extract_render_state_js(&state, interpolation);
            on_render_state.call1(&this, &render_state);

            let audio_state = extract_audio_state_js(&state, interpolation);
            on_audio_state.call1(&this, &audio_state);
            */
        }
    };

    let end = {
        let world = Rc::clone(&world);
        move |fps, abort| {
        }
    };

    //start and forget the loop
    let main_loop = MainLoop::start(MainLoopOptions::default_worker(), begin, update, draw, end)?;
    std::mem::forget(Box::new(main_loop));

    //Return the event sender
    Ok(send_event)
}


