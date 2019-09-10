use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::futures_0_3::future_to_promise;
use web_sys::{HtmlCanvasElement};
use std::rc::{Rc};
use std::cell::{RefCell};
use log::{info};
use shared::state::renderer::{State};
use shared::events::{CoreEvent, CoreEventSender, Speed};
use shared::consts;
use awsm_web::loaders::fetch;
use awsm_web::webgl::{
    get_webgl_context_1, 
    WebGlContextOptions, 
    ClearBufferMask,
    WebGlCommon,
    WebGl1Renderer,
    Id
};
use super::renderer::Renderer;

pub fn load_assets(renderer:Rc<RefCell<Renderer>>) {

    future_to_promise({
            async move {
                let vertex = fetch::text("media/shaders/vertex.glsl").await?;
                let fragment = fetch::text("media/shaders/fragment.glsl").await?;
                let image = fetch::image("media/images/smiley.svg").await?;

                let mut renderer= renderer.borrow_mut();
                let program_id = renderer.webgl.compile_program(&vertex, &fragment)?;
                renderer.program_id = Some(program_id);


                renderer.send_event(&CoreEvent::RendererLoaded);

                Ok(JsValue::null())
            }
        });
}