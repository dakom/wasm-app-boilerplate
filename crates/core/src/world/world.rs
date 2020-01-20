use shipyard::prelude::*;
use crate::consts;
use crate::components::*;

pub fn init_world(window_width: u32, window_height: u32) -> World {
    let world = World::new::<(
        Position,
        LastPosition,
        Speed,
        Direction,
        WindowSize,
        Collision
    )>();

    //These are added immediately 
    world.add_unique(AudioActive(consts::DEFAULT_AUDIO));
    world.add_unique(AssetsLoaded::default());

    {
        let (mut entities, views) = world.borrow::<(EntitiesMut, (&mut Position, &mut LastPosition, &mut Speed, &mut Direction))>();
        let (mut positions, mut last_positions, mut speeds, mut directions) = views;
        entities.add_entity(
            (&mut positions, &mut last_positions, &mut speeds, &mut directions), 
            (
                Position { x: (window_width as f64) / 2.0, y: (window_height as f64) / 2.0},
                LastPosition { x: (window_width as f64) / 2.0, y: (window_height as f64) / 2.0},
                Speed(consts::INITIAL_SPEED),
                Direction {x: 1.0, y: 1.0}
            )
        );

        let mut window_sizes = world.borrow::<&mut WindowSize>();
        entities.add_entity(&mut window_sizes, WindowSize {width: window_width, height: window_height});
    }

    world
}