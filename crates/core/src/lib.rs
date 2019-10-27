#![feature(async_await)]

mod audio;
mod components;
mod events;
mod game_loop;
mod renderer;
mod systems;
mod world;
mod consts;

use cfg_if::cfg_if;
use log::{info, Level};
use wasm_bindgen::prelude::*;
use std::rc::{Rc};
use crate::game_loop::GameLoop;
use crate::events::{handle_event};
use crate::world::init_world;

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
pub fn run(window_width: u32, window_height: u32) -> Result<JsValue, JsValue> {
    init_panic();
    init_log();

    let world = Rc::new(init_world(window_width, window_height));

    let game_loop = Box::new({
        let world = Rc::clone(&world);
        GameLoop::new(&world)?
    });
        

    //Create a function which allows JS to send us events ad-hoc
    //We will need to get a handle and forget the Closure
    //See https://stackoverflow.com/a/53219594/784519
    let _send_event = Box::new({
        let world = Rc::clone(&world);
        move |evt_type:u32, data:JsValue| {
            {
                //The actual handling of events is in this function
                match handle_event(evt_type, data, &world) {
                    Err(reason) => info!("Error: {:?}", reason),
                    _ => {}
                }
            }
        }
    }) as Box<dyn FnMut(u32, JsValue) -> ()>;

    let _send_event = Closure::wrap(_send_event);

    let send_event = _send_event.as_ref().clone();

    //forget the things that need to persist in memory 
    std::mem::forget(game_loop);
    _send_event.forget();


    Ok(send_event)
}