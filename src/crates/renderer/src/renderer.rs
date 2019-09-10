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
use super::assets::load_assets;
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
    event_sender: CoreEventSender,
    pub webgl:WebGl1Renderer,
    pub program_id: Option<Id>
}

impl Renderer {
    pub fn new(canvas:HtmlCanvasElement, send_event: js_sys::Function) -> Result<Self, JsValue> {
        //not using any webgl2 features so might as well stick with v1
        let gl = get_webgl_context_1(&canvas, Some(&WebGlContextOptions {
            alpha: false,
            ..WebGlContextOptions::default()
        }))?;

        let webgl = WebGl1Renderer::new(gl)?;

        let event_sender = CoreEventSender::new(send_event);


        Ok(Self {
            event_sender,
            webgl,
            program_id: None
        })
    }

    pub fn send_event(&self, evt:&CoreEvent) {
        self.event_sender.send(evt);
    }

    pub fn pre_render(&mut self, window_width: u32, window_height: u32) {
        //This is checked in awsm to skip if it's the same as last tick
        self.webgl.resize(window_width, window_height);
        self.webgl.gl.clear_color(1.0, 1.0, 1.0, 1.0);
        self.webgl.clear(&[
            ClearBufferMask::ColorBufferBit,
            ClearBufferMask::DepthBufferBit,
        ]);
    }

    pub fn render(&mut self, state:State) {
        self.pre_render(state.window_size.width, state.window_size.height);

        //info!("ball radius: {}, position: {:?}", consts::ball.radius, state.ball_position);
    }



}

pub fn start(canvas:HtmlCanvasElement, window_width: u32, window_height: u32, send_event:js_sys::Function) -> Result<JsValue, JsValue> {
    let mut renderer = Renderer::new(canvas, send_event)?;

    renderer.pre_render(window_width, window_height);

    // renderer.send_event(&CoreEvent::SetSpeed(Speed(0.3)));

    let renderer = Rc::new(RefCell::new(renderer));

    load_assets(Rc::clone(&renderer));

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

