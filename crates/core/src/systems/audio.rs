use shipyard::*;
use crate::components::*;
use crate::audio::AudioSequencer;

pub fn sequence(world:&World, sequencer:&mut AudioSequencer, _interpolation:f64) {

    let mut is_active = true;

    world.run::<(&AudioActive), _>(|active| {
        if let Some(active) = active.iter().next() {
            is_active = active.0;
        }
    });

    if is_active {
        world.run::<(&Collision), _>(|collision| {
            if let Some(_) = collision.iter().next() {
                sequencer.play().unwrap();
            }
        });
    }
}