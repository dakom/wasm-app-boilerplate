use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use super::{BridgeEvent};

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

    //if we needed data to come along, evt would be an enum or we could have different functions
    //like send_bridge_event() vs. send(), or use Trait Objects, etc.
    pub fn send(&self, evt:&BridgeEvent) {

        let evt_type = JsValue::from(*evt as u32);

        self._send_event.call2(&JsValue::NULL, &evt_type, &JsValue::UNDEFINED).unwrap();
    }
}