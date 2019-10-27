use shipyard::*;
use crate::consts;
use crate::components::*;
use crate::renderer::Renderer;
use std::cell::RefCell;
use std::rc::Rc;
use crate::audio::AudioSequencer;
use log::info;
use crate::events::*;

pub fn sequence(world:&World, sequencer:&mut AudioSequencer, interpolation:f64) {

    let mut is_active = true;

    world.run::<(&AudioActive), _>(|active| {
        if let Some(active) = active.iter().next() {
            is_active = active.0;
        }
    });

    if is_active {
        world.run::<(&Collision), _>(|collision| {
            if let Some(collision) = collision.iter().next() {
                sequencer.play();
            }
        });
    }
}