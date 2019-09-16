use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::futures_0_3::future_to_promise;
use web_sys::{HtmlCanvasElement};
use std::rc::{Rc};
use std::cell::{RefCell};
use log::{info};
use shared::state::*;
use shared::consts;
use super::events::*;
use super::assets::load_assets;
use nalgebra::{Matrix4, Point2, Vector2, Vector3};
use awsm_web::loaders::fetch;
use awsm_web::webgl::{
    get_webgl_context_1, 
    WebGlContextOptions, 
    ClearBufferMask,
    WebGlCommon,
    WebGl1Renderer,
    Id,
    GlToggle,
    BeginMode,
    BlendFactor
};

pub struct Renderer {
    pub webgl:WebGl1Renderer,
    pub program_id: Option<Id>,
    pub texture_id: Option<Id>,
    pub vao_id: Option<Id>,
    event_sender: EventSender,
    prev_state: Option<State>,

    camera_mat:Matrix4<f32>,
    scaling_mat:Matrix4<f32>
}

impl Renderer {
    pub fn new(canvas:HtmlCanvasElement, send_event: js_sys::Function) -> Result<Self, JsValue> {
        //not using any webgl2 features so might as well stick with v1
        let gl = get_webgl_context_1(&canvas, Some(&WebGlContextOptions{
            alpha: false,
            ..WebGlContextOptions::default()
        }))?;

        let webgl = WebGl1Renderer::new(gl)?;

        let event_sender = EventSender::new(send_event);


        let scaling_mat = Matrix4::new_nonuniform_scaling(&Vector3::new(
                    (consts::BALL.radius * 2.0) as f32,
                    (consts::BALL.radius * 2.0) as f32,
                    0.0f32,
        ));

        Ok(Self {
            event_sender,
            webgl,
            program_id: None,
            texture_id: None,
            vao_id: None,
            prev_state: None,
            camera_mat: Matrix4::identity(),
            scaling_mat
        })
    }

    pub fn send_event(&self, evt:&Event) {
        self.event_sender.send(evt);
    }

    pub fn pre_render(&mut self, window_width: u32, window_height: u32) {
        //These are checked in awsm to skip if it's the same as last tick
        self.webgl.resize(window_width, window_height);
        self.webgl.toggle(GlToggle::Blend, true);
        self.webgl.set_blend_func(BlendFactor::SrcAlpha, BlendFactor::OneMinusSrcAlpha);
        self.webgl.gl.clear_color(1.0, 1.0, 1.0, 1.0);
        self.webgl.clear(&[
            ClearBufferMask::ColorBufferBit,
            ClearBufferMask::DepthBufferBit,
        ]);
    }

    pub fn on_state(&mut self, state:State) {
        self.pre_render(state.window_width, state.window_height);

        let size_changed = match &self.prev_state {
            None => true,
            Some(prev_state) => state.window_height != prev_state.window_height || state.window_width != prev_state.window_width
        };

        if size_changed {
            self.update_camera(state.window_width, state.window_height);
        }

        self.render(&state);

        self.prev_state = Some(state);
        //info!("ball radius: {}, position: {:?}", consts::ball.radius, state.ball_position);
    }

    fn update_camera(&mut self, width: u32, height: u32) {
        self.camera_mat = Matrix4::new_orthographic(
                    0.0,
                    width as f32,
                    0.0,
                    height as f32,
                    0.0,
                    1.0,
        );
    }

    fn render(&mut self, state:&State) {
        self.webgl.activate_program(self.program_id.unwrap()).unwrap();


        let pos = match &self.prev_state {
            None => (state.ball_position_x as f32, state.ball_position_y as f32),
            Some(prev_state) => {
                let v1 = Vector2::new(prev_state.ball_position_x, prev_state.ball_position_y);
                let v2 = Vector2::new(state.ball_position_x, state.ball_position_y);
                let res = v1.lerp(&v2, state.interpolation);
                //info!("{} -> {}, {} -> {}", v2[0], res[0], v2[1], res[1]);
                (res[0] as f32, res[1] as f32)
            }
        };

        self.webgl.upload_uniform_fvals_2("u_position", pos);

        self.webgl.upload_uniform_mat_4("u_camera", &self.camera_mat.as_slice()).unwrap();

        self.webgl.upload_uniform_mat_4("u_size", &self.scaling_mat.as_slice()).unwrap();

        self.webgl.activate_texture_for_sampler(self.texture_id.unwrap(), "u_sampler").unwrap();

        self.webgl.activate_vertex_array(self.vao_id.unwrap()).unwrap();

        self.webgl.draw_arrays(BeginMode::TriangleStrip, 0, 4);
    }

}

pub fn start(canvas:HtmlCanvasElement, window_width: u32, window_height: u32, send_event:js_sys::Function) -> Result<JsValue, JsValue> {
    let mut renderer = Renderer::new(canvas, send_event)?;
 
    renderer.webgl.register_extension_vertex_array();
    renderer.pre_render(window_width, window_height);

    // renderer.send_event(&IoEvent::SetSpeed(Speed(0.3)));

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
                        if state.renderer_active {
                            renderer.on_state(state);
                        }
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