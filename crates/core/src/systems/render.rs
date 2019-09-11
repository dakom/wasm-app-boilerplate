use shipyard::*;
use shared::state::renderer;
use shared::consts;
use crate::world::{Position, Speed, Direction};

/*
#[system(Render)]
fn run(pos: &Position, speed: &Speed, dir: &Direction) {
    /*
    renderer::State {
        window_size: state.window_size,
        //The ball position is considered the midpoint for physics
        //So drawing should also expect it as the midpoint
        ball_position: Position {
            x: state.ball_position.x - consts::ball.radius,
            y: state.ball_position.y - consts::ball.radius
        },
        interpolation,
        has_loaded: state.renderer_loaded
    }
    */

    let mut ball_position = renderer::Position { x: 0.0, y: 0.0 };

    for (pos) in (pos).iter() {

    }
}

pub fn register(world:&World) {
    world.add_workload("Render", Render);
}
*/

pub fn extract_render_state(world:&World, interpolation:f64) -> renderer::State {

    let mut state = renderer::State::new();

    world.run::<(&Position), _>(|(positions)| {
        if let Some(pos) = positions.iter().next() {
            state.ball_position.x = pos.x - consts::ball.radius;
            state.ball_position.y = pos.y - consts::ball.radius;
        }
    });

    state
}