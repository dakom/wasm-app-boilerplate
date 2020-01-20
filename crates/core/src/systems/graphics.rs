use shipyard::prelude::*;
use crate::consts;
use crate::components::*;
use crate::renderer::Renderer;
use crate::events::*;
use nalgebra::{Vector2};
pub fn render(world:&World, renderer:&mut Renderer, interpolation:f64) {

    let window_sizes = world.borrow::<&WindowSize>();
    if let Some(window_size) = window_sizes.iter().next() {
        renderer.pre_render(window_size.width, window_size.height)
    }

    let (positions, last_positions) = world.borrow::<(&Position, &LastPosition)>();

    if let Some((pos, last_pos)) = (&positions, &last_positions).iter().next() {
        let prev = Vector2::new(last_pos.x - consts::BALL.radius, last_pos.y - consts::BALL.radius);
        let curr = Vector2::new(pos.x - consts::BALL.radius, pos.y - consts::BALL.radius);
        let res = prev.lerp(&curr, interpolation);
        renderer.render((res[0] as f32, res[1] as f32));
    }
}