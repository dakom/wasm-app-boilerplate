use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::rc::{Rc};
use std::cell::{RefCell};
use log::{info};
use super::events::*;
use shared::state::{State};
use web_sys::{AudioContext};
use wasm_bindgen_futures::futures_0_3::future_to_promise;
use awsm_web::loaders::fetch;
use super::assets::load_assets;

pub struct Sequencer {
    pub ctx: AudioContext,
    pub is_active: bool,
    event_sender: EventSender,
}

impl Sequencer {
    pub fn new(send_event: js_sys::Function, ctx:AudioContext) -> Result<Self, JsValue> {
        let event_sender = EventSender::new(send_event);

        Ok(Self{
            ctx,
            event_sender, 
            is_active: true
        })
    }

    pub fn send_event(&self, evt:&Event) {
        self.event_sender.send(evt);
    }

    pub fn on_state(&mut self, state:State, interpolation:f64) {
        if self.is_active != state.audio_active {
            self.is_active = state.audio_active;
            info!("audio set to: {}", state.audio_active);
        }
    }

}

pub fn start(send_event:js_sys::Function, ctx:AudioContext) -> Result<JsValue, JsValue> {
    let mut sequencer = Sequencer::new(send_event, ctx)?;

    //sequencer.send_event(&IoEvent::SetSpeed(Speed(0.3)));

    let sequencer = Rc::new(RefCell::new(sequencer));
    load_assets(Rc::clone(&sequencer));

    //Create a function which allows JS to call us for rendering
    //We will need to get a handle and forget the Closure
    //See https://stackoverflow.com/a/53219594/784519
    let _render = Closure::wrap(Box::new({
        let sequencer = Rc::clone(&sequencer);
        move |data:JsValue, interpolation:f64| {
            {
                let state:Result<State, serde_wasm_bindgen::Error> = serde_wasm_bindgen::from_value(data);
                match state {
                    Ok(state) => {
                        let mut sequencer = sequencer.borrow_mut();
                        sequencer.on_state(state, interpolation);
                    },
                    Err(reason) => info!("Error: {:?}", reason)
                }
            }
        }
    }) as Box<FnMut(JsValue, f64) -> ()>);

    let render = _render.as_ref().clone();
    _render.forget();

    //Return the event sender
    Ok(render)
}