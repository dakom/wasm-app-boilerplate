use wasm_bindgen::prelude::*;
use std::rc::{Rc};
use std::cell::{RefCell};
use log::{info};
use awsm_web::tick::{MainLoop, MainLoopOptions};
use crate::events::{handle_event};
use crate::state::{State, extract_ui_state_js, extract_render_state_js, extract_audio_state_js};

pub fn start(on_ui_state: js_sys::Function, on_render_state:js_sys::Function, on_audio_state:js_sys::Function, window_width: u32, window_height: u32) -> Result<JsValue, JsValue> {
    let state = Rc::new(RefCell::new(State::new(window_width, window_height)));

    //Create a function which allows JS to send us events ad-hoc
    //We will need to get a handle and forget the Closure
    //See https://stackoverflow.com/a/53219594/784519
    let _send_event = Closure::wrap(Box::new({
        let state = Rc::clone(&state);

        move |evt_type:u32, data:JsValue| {
            {
                let mut state = state.borrow_mut();
                //The actual handling of events is in this function
                match handle_event(evt_type, data, &mut state) {
                    Ok(_) => {},
                    Err(reason) => info!("Error: {:?}", reason)
                }
            }
        }
    }) as Box<FnMut(u32, JsValue) -> ()>);

    let send_event = _send_event.as_ref().clone();
    _send_event.forget();

    //Main loop callbacks
    let begin = move |time, delta| {
    };

    let update = {
        move |delta| {
        }
    };

    let draw = {
        let state = Rc::clone(&state);
        move |interpolation| {
            let state = state.borrow();
            let this = JsValue::NULL;
            
            let ui_state = extract_ui_state_js(&state, interpolation);
            on_ui_state.call1(&this, &ui_state);
            
            let render_state = extract_render_state_js(&state, interpolation);
            on_render_state.call1(&this, &render_state);

            let audio_state = extract_audio_state_js(&state, interpolation);
            on_audio_state.call1(&this, &audio_state);
        }
    };

    let end = move |fps, abort| {
    };

    //start and forget the loop
    let main_loop = MainLoop::start(MainLoopOptions::default_worker(), begin, update, draw, end)?;
    std::mem::forget(Box::new(main_loop));

    //Return the event sender
    Ok(send_event)
}

