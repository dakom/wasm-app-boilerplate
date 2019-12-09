use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;
use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;
use std::convert::TryFrom;
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ts_test")] {
        use strum_macros::{EnumIter, AsRefStr};
        use strum::{IntoEnumIterator};
    }
}

#[cfg_attr(feature = "ts_test", derive(EnumIter, AsRefStr))]
#[derive(FromPrimitive, Copy, Clone, Debug)]
#[repr(u32)]
pub enum BridgeEvent {
    ToggleAudio,
    Speed,
    WindowSize,
    AssetsLoaded,
    RenderUi,
    BgTexture
}

impl TryFrom<u32> for BridgeEvent {
    type Error = String;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        FromPrimitive::from_u32(value).ok_or_else(|| format!("BridgeEvent: {} is outside of range!", value))
    }
}

//All the event data:
#[derive(Serialize, Deserialize)]
pub struct Speed(pub f64);

#[derive(Serialize, Deserialize)]
pub struct Timestamp(pub f64);

#[derive(Serialize, Deserialize)]
pub struct WindowSize {
    pub width: u32,
    pub height: u32 
}

cfg_if! {
    if #[cfg(feature = "ts_test")] {
        #[wasm_bindgen]
        pub fn get_bridge_event_pairs() -> Vec<JsValue> {
            BridgeEvent::iter()
                .map(|evt| {
                    let index = evt as u32;
                    let name = evt.as_ref();
                    serde_wasm_bindgen::to_value(&(index, name)).unwrap()
                })
                .collect()
        }
    }
}
