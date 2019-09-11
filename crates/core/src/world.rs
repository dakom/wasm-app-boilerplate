use shipyard::*;

pub struct Position {pub x: f64, pub y: f64}
pub struct Speed (pub f32);
pub struct Direction {pub x: f64, pub y: f64}

pub fn init_world(window_width: u32, window_height: u32) -> World {
    let world = World::default();

    world.register::<Position>();
    world.register::<Speed>();
    world.register::<Direction>();

    world.run::<(EntitiesMut, &mut Position, &mut Speed, &mut Direction), _>(|(mut entities, mut pos, mut speed, mut dir)| {
        entities.add_entity(
            (&mut pos, &mut speed, &mut dir), 
            (
                Position { x: (window_width as f64) / 2.0, y: (window_height as f64) / 2.0},
                Speed(0.5),
                Direction {x: 1.0, y: 1.0}
            )
        );
    });

    world
}