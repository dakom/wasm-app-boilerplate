use shipyard::*;
use shared::consts;
use log::{info};
use crate::components::*;

pub fn update_motion(world:&World, delta:f64) {

    let mut entities_to_delete:Vec<Key> = vec![];

    world.run::<(&Collision), _>(|collision| {
        entities_to_delete = collision
            .iter()
            .with_id()
            .map(|(id, _)| id)
            .collect::<Vec<Key>>();
    });
    
    world.run::<(EntitiesMut, AllStorages), _>(|(mut entities, mut all_storages)| {
        for id in entities_to_delete {
            entities.delete(&mut all_storages, id);
        }
    });
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

                let mut collision = false;

                //TODO - use normal instead of just moving to edge
                if dir.x == -1.0 && ball_left < wall_left {
                    //pos.x = consts::BALL.radius;
                    dir.x = 1.0;
                    collision = true;
                }

                if dir.x == 1.0 && ball_right > wall_right {
                    //pos.x = wall_right - consts::BALL.radius;
                    dir.x = -1.0;
                    collision = true;
                }

                if dir.y == 1.0 && ball_top > wall_top{
                    //pos.y = wall_top - consts::BALL.radius;
                    dir.y = -1.0;
                    collision = true;
                }

                if dir.y == -1.0 && ball_bottom < wall_bottom {
                    //pos.y = consts::BALL.radius;
                    dir.y = 1.0;
                    collision = true;
                }

                if(collision) {
                    world.run::<(EntitiesMut, &mut Collision), _>(|(mut entities, mut collision)| {
                        entities.add_entity(
                            (&mut collision), 
                            (
                               Collision {} 
                            )
                        );
                    });
                }

            }
        }
    });
}