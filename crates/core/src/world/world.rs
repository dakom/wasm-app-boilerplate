use shipyard::*;
use crate::consts;
use crate::components::*;

pub fn init_world(window_width: u32, window_height: u32) -> World {
    let world = World::default();

    //These are added immediately 
    world.register::<Position>();
    world.register::<LastPosition>();
    world.register::<Speed>();
    world.register::<Direction>();
    world.register::<WindowSize>();
    world.register_unique(AudioActive(consts::DEFAULT_AUDIO));
    world.register_unique(AssetsLoaded::default());
    world.register::<Collision>();

    world.run::<(EntitiesMut, &mut Position, &mut LastPosition, &mut Speed, &mut Direction), _>(|(mut entities, mut pos, mut last_pos, mut speed, mut dir)| {
        entities.add_entity(
            (&mut pos, &mut last_pos, &mut speed, &mut dir), 
            (
                Position { x: (window_width as f64) / 2.0, y: (window_height as f64) / 2.0},
                LastPosition { x: (window_width as f64) / 2.0, y: (window_height as f64) / 2.0},
                Speed(consts::INITIAL_SPEED),
                Direction {x: 1.0, y: 1.0}
            )
        );
    });

    world.run::<(EntitiesMut, &mut WindowSize), _>(|(mut entities, mut window_size)| {
        entities.add_entity(&mut window_size, WindowSize {width: window_width, height: window_height});
    });

    world
}