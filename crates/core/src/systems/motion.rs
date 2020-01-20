use float_cmp::approx_eq;
use shipyard::prelude::*;
//use log::{info};
use crate::consts;
use crate::components::*;

pub fn update_motion(world:&World, delta:f64) {

    let entities_to_delete:Vec<EntityId> = {
        world.borrow::<&Collision>()
                .iter()
                .with_id()
                .map(|(id, _)| id)
                .collect()
    };
  
    {
        let mut all_storages = world.borrow::<AllStorages>();
        for id in entities_to_delete {
            all_storages.delete(id);
        }
    }

    let (mut positions, mut last_positions, mut directions) = world.borrow::<(&mut Position, &mut LastPosition, &mut Direction)>();
    let (speeds, window_sizes) = world.borrow::<(&Speed, &WindowSize)>();

    (&mut positions, &mut last_positions)
        .iter()
        .for_each(|(pos, last_pos)| {
            last_pos.x = pos.x; 
            last_pos.y = pos.y; 
        });

    (&mut positions, &speeds, &directions)
        .iter() 
        .for_each(|(pos, speed, dir)| { 
            let speed = speed.0;
            pos.x += (speed * dir.x) * delta;
            pos.y += (speed * dir.y) * delta;
        });

    if let Some(window_size) = window_sizes.iter().next() {
        (&mut positions, &mut directions).iter().for_each(|(pos, dir)| {


            let ball_left = pos.x - consts::BALL.radius;
            let ball_right = pos.x + consts::BALL.radius;
            let ball_top = pos.y + consts::BALL.radius;
            let ball_bottom = pos.y - consts::BALL.radius;

            let wall_left = 0.0;
            let wall_right = window_size.width as f64;
            let wall_top = window_size.height as f64;
            let wall_bottom = 0.0;

            let mut collision = false;

            //TODO - use normal instead of just moving to edge
            if approx_eq!(f64, dir.x, -1.0) && ball_left < wall_left {
                //pos.x = consts::BALL.radius;
                dir.x = 1.0;
                collision = true;
            }

            if approx_eq!(f64, dir.x, 1.0) && ball_right > wall_right {
                //pos.x = wall_right - consts::BALL.radius;
                dir.x = -1.0;
                collision = true;
            }

            if approx_eq!(f64, dir.y, 1.0) && ball_top > wall_top{
                //pos.y = wall_top - consts::BALL.radius;
                dir.y = -1.0;
                collision = true;
            }

            if approx_eq!(f64, dir.y, -1.0) && ball_bottom < wall_bottom {
                //pos.y = consts::BALL.radius;
                dir.y = 1.0;
                collision = true;
            }

            if collision {
                let (mut entities, mut collisions) = world.borrow::<(EntitiesMut, &mut Collision)>();
                entities.add_entity(&mut collisions,  Collision {});
            }

        });
    }
}