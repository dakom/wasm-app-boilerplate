use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
use std::rc::{Rc};
use std::cell::{RefCell};
use awsm_web::loaders::fetch;
use awsm_web::webgl::{
    TextureTarget,
    SimpleTextureOptions,
    PixelFormat,
    WebGlTextureSource,
    BufferData,
    BufferTarget,
    BufferUsage,
    DataType,
    AttributeOptions,
    VertexArray
};
use crate::components::*;
use super::renderer::Renderer;
use shipyard::prelude::*;

pub fn load_assets(renderer:Rc<RefCell<Renderer>>, world:Rc<World>) {


    future_to_promise({
            async move {
                //load everything - then we can borrow renderer mutably since it'll be sync
                let smiley_vertex = fetch::text("media/shaders/smiley_vertex.glsl").await?;
                let bg_vertex = fetch::text("media/shaders/bg_vertex.glsl").await?;
                let fragment = fetch::text("media/shaders/fragment.glsl").await?;
                let image = fetch::image("media/images/smiley.svg").await?;


                let mut renderer= renderer.borrow_mut();

                //PROGRAM
                let program_id = renderer.webgl.compile_program(&smiley_vertex, &fragment)?;
                renderer.smiley_program_id = Some(program_id);
                let program_id = renderer.webgl.compile_program(&bg_vertex, &fragment)?;
                renderer.bg_program_id = Some(program_id);

                //TEXTURE
                let texture_id = renderer.webgl.create_texture()?;
                renderer.webgl.assign_simple_texture(
                        texture_id,
                        TextureTarget::Texture2d,
                        &SimpleTextureOptions {
                            pixel_format: PixelFormat::Rgba,
                            ..SimpleTextureOptions::default()
                        },
                        &WebGlTextureSource::ImageElement(&image),
                    )?;
                renderer.smiley_texture_id = Some(texture_id);

                //QUAD GEOM DATA 
                let buffer_id = renderer.webgl.create_buffer()?;

                renderer.webgl.upload_buffer(
                    buffer_id,
                    BufferData::new(
                        &QUAD_GEOM_UNIT,
                        BufferTarget::ArrayBuffer,
                        BufferUsage::StaticDraw,
                    ),
                )?;

                //VAO
                let vao_id = renderer.webgl.create_vertex_array()?;
                renderer.vao_id = Some(vao_id);

                renderer.webgl.assign_vertex_array(
                        vao_id,
                        None,
                        &[VertexArray {
                            attribute_name: "a_vertex",
                            buffer_id,
                            opts: &AttributeOptions::new(2, DataType::Float),
                        }],
                    )?;




                world.run::<Unique<&mut AssetsLoaded>, _, _>(|assets_loaded| {
                    assets_loaded.renderer = true;
                });

                Ok(JsValue::null())
            }
        });
}

static QUAD_GEOM_UNIT: [f32; 8] = [
    0.0, 1.0, // top-left
    0.0, 0.0, //bottom-left
    1.0, 1.0, // top-right
    1.0, 0.0, // bottom-right
];