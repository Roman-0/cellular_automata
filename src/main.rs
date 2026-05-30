use std::any::Any;

use minifb::{Window, WindowOptions};

mod sim_behavior;
mod sim_data;
mod sim_helper;
mod simulations;

use crate::sim_behavior::Simulation;
use crate::sim_data::SimData;
use crate::sim_helper::*;
use crate::simulations::base_example::*;
const WIDTH: usize = 640;
const HEIGHT: usize = 480;

const TARGET_FPS: usize = 200;

fn main() {
    let mut window = Window::new(
        "Simple Blue Box",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            scale: minifb::Scale::X2,
            scale_mode: minifb::ScaleMode::AspectRatioStretch,
            ..WindowOptions::default()
        },
    )
    .unwrap();

    let behavior = BehaviorBase;
    let mut data = SimData::new(WIDTH, HEIGHT);
    data.set_grids(&init_random_color(WIDTH, HEIGHT));
    let mut simulation = Simulation::new(behavior, data);

    window.set_target_fps(TARGET_FPS);

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        simulation.step();

        window
            .update_with_buffer(&simulation.output().as_slice(), WIDTH, HEIGHT)
            .unwrap();
    }
}
