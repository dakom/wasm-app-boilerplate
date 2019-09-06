use wasm_bindgen::prelude::*;
use serde::Serialize;
use crate::state::{State};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Audio {
    interpolation: f64
}

impl Audio {

    pub fn new(state:&State, interpolation: f64) -> Self {
        Self {
            interpolation
        }
    }
    pub fn to_js(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self).unwrap()
    }
}