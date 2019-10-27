use wasm_bindgen::prelude::*;
use shipyard::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::futures_0_3::future_to_promise;
use web_sys::{HtmlCanvasElement};
use std::rc::{Rc};
use std::cell::{RefCell};
use log::{info};
use crate::consts;
use crate::events::*;
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

        let mut webgl = WebGl1Renderer::new(gl)?;

        webgl.register_extension_vertex_array();

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


    pub fn update_camera(&mut self, width: u32, height: u32) {
        self.camera_mat = Matrix4::new_orthographic(
                    0.0,
                    width as f32,
                    0.0,
                    height as f32,
                    0.0,
                    1.0,
        );
    }

    fn render(&mut self, world:&World, interpolation: f64) {
        //self.pre_render(state.window_width, state.window_height);
        /*
        self.webgl.activate_program(self.program_id.unwrap()).unwrap();


        let pos = match &self.prev_state {
            None => (state.ball_position_x as f32, state.ball_position_y as f32),
            Some(prev_state) => {
                let v1 = Vector2::new(prev_state.ball_position_x, prev_state.ball_position_y);
                let v2 = Vector2::new(state.ball_position_x, state.ball_position_y);
                let res = v1.lerp(&v2, interpolation);
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
        */
    }

}