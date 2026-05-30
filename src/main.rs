use minifb::{Window, WindowOptions};
use std::time::{Duration, Instant};
mod sim;
mod simulations;

use crate::sim::{Simulation, SimData};
use crate::simulations::base_example::*;

const WIDTH: usize = 320;
const HEIGHT: usize = 240;

const TARGET_FPS: usize = 60;

fn main() {
    // 1. Create a simple window
    let mut window = Window::new(
        "Simple Blue Box",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).unwrap();

    let behavior = BehaviorBase;
    let mut data = SimData::new(WIDTH, HEIGHT);
    let mut simulation = Simulation::new(behavior, data);


    window.set_target_fps(TARGET_FPS);

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        
        simulation.step();

        window
            .update_with_buffer(&simulation.output().as_slice(), WIDTH, HEIGHT)
            .unwrap();

        
    }
}
