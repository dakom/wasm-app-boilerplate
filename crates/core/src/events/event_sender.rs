use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use super::{BridgeEventIndex};

//when going from rust - use enums and only what we need!
#[derive(Serialize, Deserialize)]
pub enum Event {
    AssetsLoaded,
    RenderUi
}

#[derive(Clone)]
pub struct EventSender {
    _send_event: js_sys::Function,
}

impl EventSender {
    pub fn new(send_event:js_sys::Function) -> Self {
        Self{
            _send_event: send_event
        }
    }

    pub fn send(&self, evt:&Event) {
        let (evt_type, evt_data) = match evt {
            Event::AssetsLoaded => (BridgeEventIndex::AssetsLoaded, JsValue::UNDEFINED),
            Event::RenderUi => (BridgeEventIndex::RenderUi, JsValue::UNDEFINED),
        };

        //Even though we're ultimately going from Rust -> rustc
        //We're going by way of a worker which uses plain JS objects
        //In the future maybe we can do shared memory!

        let evt_type:u32 = evt_type as u32;
        let evt_type = JsValue::from(evt_type);

        let this = JsValue::NULL;
        self._send_event.call2(&this, &evt_type, &evt_data);
    }
}