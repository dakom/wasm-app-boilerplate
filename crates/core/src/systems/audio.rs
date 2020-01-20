use shipyard::prelude::*;
use crate::components::*;
use crate::audio::AudioSequencer;

pub fn sequence(world:&World, sequencer:&mut AudioSequencer, _interpolation:f64) {

    let (collisions, audio_active) = world.borrow::<(&Collision, Unique<&AudioActive>)>();
    if audio_active.0 && collisions.iter().next().is_some() {
        sequencer.play().unwrap();
    }
}