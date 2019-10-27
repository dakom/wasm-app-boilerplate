use awsm_web::tick;
use awsm_web::tick::{MainLoop, MainLoopOptions, RafLoop};
use log::info;
use std::cell::RefCell;
use std::rc::Rc;
use shipyard::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub struct GameLoop {
    raf_loop:RafLoop
}

impl GameLoop {
    pub fn new(world:&World) -> Result<Self, JsValue> {
    //callbacks
        let begin = move |time, delta| {
            //let my_str = format!("begin time: {} delta: {}!", time, delta);
        };

        let update = move |delta| {
        };

        let draw = move |interpolation| {

            //let this = JsValue::NULL;
            //on_state.call1(&this, &serde_wasm_bindgen::to_value(&state).unwrap()).unwrap();
            //on_state
        };
        let end = move |fps, abort| {
        };

        let raf_loop = RafLoop::start({
            let mut main_loop = MainLoop::new(MainLoopOptions::default(), begin, update, draw, end);
            move |ts| {
                main_loop.tick(ts);
            }
        })?;

        Ok(Self{
            raf_loop
        })
    }
}