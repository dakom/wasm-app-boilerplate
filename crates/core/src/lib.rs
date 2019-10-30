#[allow(clippy::module_inception)]
mod audio;
#[allow(clippy::module_inception)]
mod components;
#[allow(clippy::module_inception)]
mod events;
#[allow(clippy::module_inception)]
mod game_loop;
#[allow(clippy::module_inception)]
mod renderer;
#[allow(clippy::module_inception)]
mod systems;
#[allow(clippy::module_inception)]
mod world;
#[allow(clippy::module_inception)]
mod consts;

use cfg_if::cfg_if;
use log::{info, Level};
use wasm_bindgen::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
use crate::game_loop::GameLoop;
use crate::events::{handle_event, EventSender};
use crate::world::init_world;
use crate::renderer::Renderer;
use crate::audio::AudioSequencer;
use web_sys::{HtmlCanvasElement, AudioContext};

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
pub fn run(canvas:HtmlCanvasElement, audio_ctx:AudioContext, window_width: u32, window_height: u32, send_bridge_event:js_sys::Function) -> Result<JsValue, JsValue> {
    init_panic();
    init_log();

    let event_sender = EventSender::new(send_bridge_event);
    let world = Rc::new(init_world(window_width, window_height));
    let renderer = Renderer::new(canvas)?;
    let renderer = Rc::new(RefCell::new(renderer));
    let sequencer= AudioSequencer::new(audio_ctx)?;
    let sequencer = Rc::new(RefCell::new(sequencer));

    let game_loop = Box::new({
        let world = Rc::clone(&world);
        let renderer = Rc::clone(&renderer);
        let sequencer = Rc::clone(&sequencer);
        GameLoop::new(world, renderer, sequencer, event_sender)?
    });
        

    //Create a function which allows JS to send us events ad-hoc
    //We will need to get a handle and forget the Closure
    //See https://stackoverflow.com/a/53219594/784519
    let _send_event = Box::new({
        let world = Rc::clone(&world);
        let renderer = Rc::clone(&renderer);
        let sequencer = Rc::clone(&sequencer);
        move |evt_type:u32, data:JsValue| {
            {
                let mut renderer = renderer.borrow_mut();
                let mut sequencer = sequencer.borrow_mut();
                //The actual handling of events is in this function
                if let Err(reason) = handle_event(evt_type, data, &world, &mut renderer, &mut sequencer) {
                    info!("Error: {:?}", reason);
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