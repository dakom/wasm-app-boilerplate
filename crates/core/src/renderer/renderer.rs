use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement};
use crate::consts;
use nalgebra::{Matrix4, Vector3};
use awsm_web::webgl::{
    get_webgl_context_1, 
    WebGlContextOptions, 
    ClearBufferMask,
    WebGl1Renderer,
    Id,
    GlToggle,
    BeginMode,
    BlendFactor,
    TextureTarget,
    SimpleTextureOptions,
    PixelFormat,
    WebGlTextureSource,
};

pub struct Renderer {
    pub webgl:WebGl1Renderer,
    pub smiley_program_id: Option<Id>,
    pub bg_program_id: Option<Id>,
    pub smiley_texture_id: Option<Id>,
    pub bg_texture_id: Option<Id>,
    pub vao_id: Option<Id>,
    last_window_width: u32,
    last_window_height: u32,
    camera_mat:Matrix4<f32>,
    scaling_mat:Matrix4<f32>,
}

impl Renderer {
    pub fn new(canvas:HtmlCanvasElement) -> Result<Self, JsValue> {
        //not using any webgl2 features so might as well stick with v1
        let gl = get_webgl_context_1(&canvas, Some(&WebGlContextOptions{
            alpha: false,
            ..WebGlContextOptions::default()
        }))?;

        let mut webgl = WebGl1Renderer::new(gl)?;

        webgl.register_extension_vertex_array()?;


        let scaling_mat = Matrix4::new_nonuniform_scaling(&Vector3::new(
                    (consts::BALL.radius * 2.0) as f32,
                    (consts::BALL.radius * 2.0) as f32,
                    0.0f32,
        ));

        let bg_texture_id = webgl.create_texture()?;

        Ok(Self {
            webgl,
            smiley_program_id: None,
            bg_program_id: None,
            smiley_texture_id: None,
            bg_texture_id: Some(bg_texture_id),
            vao_id: None,
            last_window_width: 0,
            last_window_height: 0,
            camera_mat: Matrix4::identity(),
            scaling_mat
        })
    }

    pub fn pre_render(&mut self, window_width: u32, window_height: u32) {

        //These are checked in awsm to skip if it's the same as last tick
        self.webgl.resize(window_width, window_height);
        //self.webgl.set_depth_mask(false);
        self.webgl.set_depth_mask(false);
        self.webgl.toggle(GlToggle::Blend, true);
        self.webgl.set_blend_func(BlendFactor::SrcAlpha, BlendFactor::OneMinusSrcAlpha);
        self.webgl.gl.clear_color(1.0, 1.0, 1.0, 1.0);
        self.webgl.clear(&[
            ClearBufferMask::ColorBufferBit,
            ClearBufferMask::DepthBufferBit,
        ]);

        if self.last_window_width != window_width || self.last_window_height != window_height {
            self.camera_mat = Matrix4::new_orthographic(
                        0.0,
                        window_width as f32,
                        0.0,
                        window_height as f32,
                        0.0,
                        1.0,
            );
            self.last_window_width = window_width;
            self.last_window_height = window_height;
        }
    }

    pub fn render(&mut self, pos:(f32, f32)) {
        match (self.bg_program_id, self.smiley_program_id) {
            (Some(bg_program_id), Some(smiley_program_id)) => {

                self.webgl.activate_program(smiley_program_id).unwrap();
                self.webgl.upload_uniform_fvals_2("u_position", pos).unwrap();
                self.webgl.upload_uniform_mat_4("u_camera", &self.camera_mat.as_slice()).unwrap();
                self.webgl.upload_uniform_mat_4("u_size", &self.scaling_mat.as_slice()).unwrap();
                self.webgl.activate_texture_for_sampler(self.smiley_texture_id.unwrap(), "u_sampler").unwrap();
                self.webgl.activate_vertex_array(self.vao_id.unwrap()).unwrap();
                self.webgl.draw_arrays(BeginMode::TriangleStrip, 0, 4);

                self.webgl.activate_program(bg_program_id).unwrap();
                self.webgl.activate_texture_for_sampler(self.bg_texture_id.unwrap(), "u_sampler").unwrap();
                self.webgl.activate_vertex_array(self.vao_id.unwrap()).unwrap();
                self.webgl.draw_arrays(BeginMode::TriangleStrip, 0, 4);
            },
            _ => {}
        }
    }

    pub fn upload_bg_texture(&mut self, img_data:&web_sys::ImageData) -> Result<(), JsValue> {
        self.webgl.assign_simple_texture(
                self.bg_texture_id.unwrap(),
                TextureTarget::Texture2d,
                &SimpleTextureOptions {
                    pixel_format: PixelFormat::Rgba,
                    ..SimpleTextureOptions::default()
                },
                &WebGlTextureSource::ImageData(&img_data),
            )?;
        Ok(())
    }

}