use shipyard::*;
use shared::consts;
use crate::components::*;
use shared::state::*;

pub fn init_world(window_width: u32, window_height: u32) -> World {
    let world = World::default();

    //These are added immediately 
    world.register::<Position>();
    world.register::<Speed>();
    world.register::<Direction>();
    world.register::<WindowSize>();
    world.register::<AudioActive>();
    world.register::<InitState>();

    world.run::<(EntitiesMut, &mut InitState), _>(|(mut entities, mut init_state)| {
        entities.add_entity(
            (&mut init_state), 
            (
                InitState::new()
            )
        );
    });

    world.run::<(EntitiesMut, &mut Position, &mut Speed, &mut Direction), _>(|(mut entities, mut pos, mut speed, mut dir)| {
        entities.add_entity(
            (&mut pos, &mut speed, &mut dir), 
            (
                Position { x: (window_width as f64) / 2.0, y: (window_height as f64) / 2.0},
                Speed(consts::INITIAL_SPEED),
                Direction {x: 1.0, y: 1.0}
            )
        );
    });

    world.run::<(EntitiesMut, &mut WindowSize), _>(|(mut entities, mut window_size)| {
        entities.add_entity(
            (&mut window_size), 
            (
                WindowSize {width: window_width, height: window_height} 
            )
        );
    });


    world.run::<(EntitiesMut, &mut AudioActive), _>(|(mut entities, mut audio_active)| {
        entities.add_entity(
            (&mut audio_active), 
            (
                AudioActive(State::default().audio_active)
            )
        );
    });
    world
}