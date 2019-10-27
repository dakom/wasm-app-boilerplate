use shipyard::*;
use crate::consts;
use crate::components::*;
use crate::renderer::Renderer;
use std::cell::RefCell;
use std::rc::Rc;
use crate::audio::AudioSequencer;
use log::info;
use crate::events::*;
use nalgebra::{Matrix4, Point2, Vector2, Vector3};
pub fn render(world:&World, renderer:&mut Renderer, interpolation:f64) {

    world.run::<(&WindowSize), _>(|window_size| {
        if let Some(window_size) = window_size.iter().next() {
            renderer.pre_render(window_size.width, window_size.height)
        }
    });


    world.run::<(&Position, &LastPosition), _>(|(pos, last_pos)| {
        if let Some((pos, last_pos)) = (pos, last_pos).iter().next() {
            let prev = Vector2::new(last_pos.x - consts::BALL.radius, last_pos.y - consts::BALL.radius);
            let curr = Vector2::new(pos.x - consts::BALL.radius, pos.y - consts::BALL.radius);
            let res = prev.lerp(&curr, interpolation);
            renderer.render((res[0] as f32, res[1] as f32));
        }
    });
}