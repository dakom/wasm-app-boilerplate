use shipyard::*;
use crate::components::*;
use crate::audio::AudioSequencer;

pub fn sequence(world:&World, sequencer:&mut AudioSequencer, _interpolation:f64) {

    world.run::<(&Collision, Unique<&AudioActive>), _>(|(collision, is_active)| {
        if is_active.0 && collision.iter().next().is_some() {
            sequencer.play().unwrap();
        }
    });
}