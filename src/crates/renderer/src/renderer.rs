use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement};
use std::rc::{Rc};
use std::cell::{RefCell};
use log::{info};
use shared::renderer::{State};

use wasm_bindgen_futures::futures_0_3::future_to_promise;
use awsm_web::loaders::fetch;
use awsm_web::webgl::{
    get_webgl_context_1, 
    WebGlContextOptions, 
    ClearBufferMask,
    WebGlCommon,
    WebGl1Renderer,
    Id
};


pub struct Renderer {
    renderer:WebGl1Renderer,
    program_id: Option<Id>
}

impl Renderer {
    pub fn new(canvas:HtmlCanvasElement) -> Result<Self, JsValue> {
        //not using any webgl2 features so might as well stick with v1
        let gl = get_webgl_context_1(&canvas, Some(&WebGlContextOptions {
            alpha: false,
            ..WebGlContextOptions::default()
        }))?;

        let renderer = WebGl1Renderer::new(gl)?;



        Ok(Self {
            renderer,
            program_id: None
        })
/*
        future_to_promise({
            let _self = Rc::clone(&_self);
            async move {
                let vertex = fetch::text("media/vertex.glsl").await?;
                let fragment = fetch::text("media/fragment.glsl").await?;

                let mut _self = _self.borrow_mut();
                let program_id = _self.renderer.compile_program(&vertex, &fragment)?;
                _self.program_id = Some(program_id);

                Ok(JsValue::null())
            }
        });
*/
    }

    fn pre_render(&mut self, window_width: u32, window_height: u32) {
        //This is checked in awsm to skip if it's the same as last tick
        self.renderer.resize(window_width, window_height);
        self.renderer.gl.clear_color(1.0, 1.0, 1.0, 1.0);
        self.renderer.clear(&[
            ClearBufferMask::ColorBufferBit,
            ClearBufferMask::DepthBufferBit,
        ]);
    }

    pub fn render(&mut self, state:State) {
        self.pre_render(state.window_size.width, state.window_size.height);
    }

}

pub fn start(canvas:HtmlCanvasElement, window_width: u32, window_height: u32) -> Result<JsValue, JsValue> {
    let mut renderer = Renderer::new(canvas)?;

    renderer.pre_render(window_width, window_height);

    let renderer = Rc::new(RefCell::new(renderer));

    //Create a function which allows JS to call us for rendering
    //We will need to get a handle and forget the Closure
    //See https://stackoverflow.com/a/53219594/784519
    let _render = Closure::wrap(Box::new({
        let renderer = Rc::clone(&renderer);
        move |data:JsValue| {
            {
                let state:Result<State, serde_wasm_bindgen::Error> = serde_wasm_bindgen::from_value(data);
                match state {
                    Ok(state) => {
                        let mut renderer = renderer.borrow_mut();
                        renderer.render(state);
                    },
                    Err(reason) => info!("Error: {:?}", reason)
                }
            }
        }
    }) as Box<FnMut(JsValue) -> ()>);

    let render = _render.as_ref().clone();
    _render.forget();

    //Return the event sender
    Ok(render)
}

