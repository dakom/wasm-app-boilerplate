use awsm_web::tick;
use awsm_web::tick::{MainLoop, MainLoopOptions, RafLoop};
use crate::renderer::Renderer;
use crate::audio::AudioSequencer;
use crate::events::*;
use crate::systems;

use log::info;
use std::cell::RefCell;
use std::rc::Rc;
use shipyard::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub struct GameLoop {
    raf_loop:RafLoop
}

impl GameLoop {
    pub fn new(world:Rc<World>, renderer:Rc<RefCell<Renderer>>, sequencer:Rc<RefCell<AudioSequencer>>, event_sender:EventSender) -> Result<Self, JsValue> {

        let begin = {
            let event_sender = event_sender.clone();
            let world = Rc::clone(&world);

            move |_time, _delta| {
                systems::loaders::update_loaders(&world, &event_sender);
            }
        };

        let update = {
            let world = Rc::clone(&world);
            move |delta| {
                systems::motion::update_motion(&world, delta);
            }
        };

        let draw = {
            let renderer = Rc::clone(&renderer);
            let sequencer = Rc::clone(&sequencer);
            let world = Rc::clone(&world);
            move |interpolation| {
                let mut renderer = renderer.borrow_mut();
                let mut sequencer = sequencer.borrow_mut();
                systems::graphics::render(&world, &mut renderer, interpolation);
                systems::audio::sequence(&world, &mut sequencer, interpolation);
                event_sender.send(&Event::RenderUi{});
            }
        };

        let end = {
            move |_fps, _abort| {
            }
        };

        let raf_loop = RafLoop::start({
            let mut main_loop = MainLoop::new(MainLoopOptions::default(), begin, update, draw, end);
            move |ts| {
                main_loop.tick(ts);
            }
        })?;


        //Initialize loaders
        //TODO: this would be nicer as part of the game loop itself
        //But that gave lifetime errors...
        crate::audio::assets::load_assets(sequencer, Rc::clone(&world));
        crate::renderer::assets::load_assets(renderer, Rc::clone(&world));

        Ok(Self{
            raf_loop
        })
    }
}