use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use super::from_js::{CoreEventIndex};
pub use super::data::*;

//when going from rust - use enums!
#[derive(Serialize, Deserialize)]
pub enum CoreEvent {
    ToggleAudio,
    SetSpeed(Speed),
    WindowSize,
    RendererLoaded,
    AudioLoaded,
    Started
}

pub struct CoreEventSender {
    _send_event: js_sys::Function,
}

impl CoreEventSender {
    pub fn new(send_event:js_sys::Function) -> Self {
        Self{
            _send_event: send_event
        }
    }

    pub fn send(&self, evt:&CoreEvent) {
        let (evt_type, evt_data) = match evt {
            CoreEvent::ToggleAudio => (CoreEventIndex::ToggleAudio, None),
            CoreEvent::SetSpeed(data) => (CoreEventIndex::SetSpeed, Some(data)),
            CoreEvent::WindowSize => (CoreEventIndex::WindowSize, None),
            CoreEvent::RendererLoaded=> (CoreEventIndex::RendererLoaded, None),
            CoreEvent::AudioLoaded => (CoreEventIndex::AudioLoaded, None),
            CoreEvent::Started => (CoreEventIndex::Started, None)
        };

        //Even though we're ultimately going from Rust -> rustc
        //We're going by way of a worker which uses plain JS objects
        //In the future maybe we can do shared memory!

        let evt_type:u32 = evt_type as u32;
        let evt_type = JsValue::from(evt_type);

        let evt_data = serde_wasm_bindgen::to_value(&evt_data).unwrap();
        let this = JsValue::NULL;
        self._send_event.call2(&this, &evt_type, &evt_data);
    }
}