use wasm_bindgen::prelude::*;
use serde::Serialize;
use crate::state::{State};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Ui <'a> {
    text_input: &'a str,
    results: &'a [String]
}

impl <'a> Ui <'a> {

    pub fn new(state:&'a State) -> Self {
        Self {
            text_input: &state.text_input,
            results: &state.results
        }
    }
    pub fn to_js(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self).unwrap()
    }
}