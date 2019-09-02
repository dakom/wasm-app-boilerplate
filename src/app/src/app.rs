use wasm_bindgen::prelude::*;
use std::rc::{Rc};
use std::cell::{RefCell};
use log::{info};
use crate::events::{handle_event};
use crate::state::{State};
use crate::ui::{Ui};

pub fn start(on_ui_state: js_sys::Function) -> Result<JsValue, JsValue> {
    let state = Rc::new(RefCell::new(State::new()));

    //First - state is used as a source for ui_state which will be sent to the ui thread
    let render = {
        let state = Rc::clone(&state);
        move || {
            let state = state.borrow();
            let this = JsValue::NULL;
            let ui_state = Ui::new(&state);
            on_ui_state.call1(&this, &ui_state.to_js());
        }
    };

    render(); // initial render

    //Second - state is used as a mutable destination to be changed when events come in from the ui thread
    let _send_ui_event = Closure::wrap(Box::new({
        let state = Rc::clone(&state);

        move |evt_type:u32, data:JsValue| {
            {
                let mut state = state.borrow_mut();
                match handle_event(evt_type, data, &mut state) {
                    Ok(_) => {},
                    Err(reason) => info!("Error: {:?}", reason)
                }
            }
            render();
        }
    }) as Box<FnMut(u32, JsValue) -> ()>);

    //It must be returned to JS, which means it would be Dropped here
    //So we need to get a handle and forget the Closure
    //See https://stackoverflow.com/a/53219594/784519
    let send_ui_event = _send_ui_event.as_ref().clone();

    _send_ui_event.forget();

    Ok(send_ui_event)
}

