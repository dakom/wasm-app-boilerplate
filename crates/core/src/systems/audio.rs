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

    world.run::<(&Collision, Unique<&AudioActive>), _>(|(collision, is_active)| {
        if is_active.0 && collision.iter().next().is_some() {
            sequencer.play();
        }
    });
}