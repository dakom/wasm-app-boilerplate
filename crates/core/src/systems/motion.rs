use float_cmp::approx_eq;
use shipyard::prelude::*;
//use log::{info};
use crate::consts;
use crate::components::*;

pub fn update_motion(world:&World, delta:f64) {

    let mut entities_to_delete:Vec<Key> = vec![];

    world.run::<&Collision, _, _>(|collision| {
        entities_to_delete = collision
            .iter()
            .with_id()
            .map(|(id, _)| id)
            .collect::<Vec<Key>>();
    });
    
    world.run::<AllStorages, _, _>(|mut all_storages| {
        for id in entities_to_delete {
            all_storages.delete(id);
        }
    });


    world.run::<(&Position, &mut LastPosition), _, _> (|(pos, last_pos)| {
        for (pos, last_pos) in (pos, last_pos).iter() { 
            last_pos.x = pos.x; 
            last_pos.y = pos.y; 
        }
    });

    world.run::<(&mut Position, &Speed, &Direction), _, _> (|(positions, speeds, directions)| {
        for (pos, speed, dir) in (positions, speeds, directions).iter() { 
            let speed = speed.0;
            pos.x += (speed * dir.x) * delta;
            pos.y += (speed * dir.y) * delta;

        }
    });

    #[allow(clippy::useless_let_if_seq)]
    world.run::<(&mut Position, &mut Direction, &WindowSize), _, _> (|(positions, directions, window_sizes)| {
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
                    world.run::<(EntitiesMut, &mut Collision), _, _>(|(mut entities, mut collision)| {
                        entities.add_entity(&mut collision,  Collision {});
                    });
                }

            }
        }
    });


}