use shipyard::*;
use std::rc::Rc;
use awsm_web::tick::{MainLoop, MainLoopOptions};
use crate::systems::render::{extract_render_state};
use crate::systems::ui::{extract_ui_state};
use crate::systems::audio::{extract_audio_state};
use shared::state::renderer;
use shared::state::audio;
use shared::state::ui;
use wasm_bindgen::prelude::*;

pub fn start(world:Rc<World>, on_ui_state: js_sys::Function, on_render_state:js_sys::Function, on_audio_state:js_sys::Function) -> Result<(), JsValue> {
    //Main loop callbacks
    let begin = {
        let world = Rc::clone(&world);
        move |time, delta| {
        }
    };

    let update = {
        let world = Rc::clone(&world);
        move |delta| {
        }
    };

    let draw = {
        let world = Rc::clone(&world);
        let mut render_state = renderer::State::new();
        let mut ui_state = ui::State::new();
        let mut audio_state = audio::State::new();

        move |interpolation| {
            let this = JsValue::NULL;

            extract_render_state(&world, interpolation, &mut render_state);
            on_render_state.call1(&this, &serde_wasm_bindgen::to_value(&render_state).unwrap());

            extract_ui_state(&world, interpolation, &mut ui_state);
            on_ui_state.call1(&this, &serde_wasm_bindgen::to_value(&ui_state).unwrap());

            extract_audio_state(&world, interpolation, &mut audio_state);
            on_audio_state.call1(&this, &serde_wasm_bindgen::to_value(&audio_state).unwrap());
        }
    };

    let end = {
        let world = Rc::clone(&world);
        move |fps, abort| {
        }
    };

    //start and forget the loop
    let main_loop = MainLoop::start(MainLoopOptions::default_worker(), begin, update, draw, end)?;
    std::mem::forget(Box::new(main_loop));

    Ok(())
}