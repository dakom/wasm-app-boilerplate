use shipyard::*;
use shared::consts;
use log::{info};
use crate::components::*;

pub fn update_motion(world:&World, delta:f64) {

    world.run::<(&mut Position, &Speed, &Direction), _> (|(positions, speeds, directions)| {
        for (pos, speed, dir) in (positions, speeds, directions).iter() { 
            let speed = speed.0;
            pos.x += (speed * dir.x) * delta;
            pos.y += (speed * dir.y) * delta;

        }
    });

    world.run::<(&mut Position, &mut Direction, &WindowSize), _> (|(positions, directions, window_sizes)| {
        if let Some(window_size) = window_sizes.iter().next() {
            for (pos, dir) in (positions, directions).iter() {


                let ball_left = pos.x - consts::BALL.radius;
                let ball_right = pos.x + consts::BALL.radius;
                let ball_top = pos.y + consts::BALL.radius;
                let ball_bottom = pos.y - consts::BALL.radius;

                let wall_left = 0.0;
                let wall_right = window_size.width as f64;
                let wall_top = window_size.height as f64;
                let wall_bottom = 0.0;

                //TODO - use normal instead of just moving to edge
                if dir.x == -1.0 && ball_left < wall_left {
                    //pos.x = consts::BALL.radius;
                    dir.x = 1.0;
                }

                if dir.x == 1.0 && ball_right > wall_right {
                    //pos.x = wall_right - consts::BALL.radius;
                    dir.x = -1.0;
                }

                if dir.y == 1.0 && ball_top > wall_top{
                    //pos.y = wall_top - consts::BALL.radius;
                    dir.y = -1.0;
                }

                if dir.y == -1.0 && ball_bottom < wall_bottom {
                    //pos.y = consts::BALL.radius;
                    dir.y = 1.0;
                }
            }
        }
    });
}