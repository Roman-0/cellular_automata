use std::any::Any;

use minifb::{Window, WindowOptions};

mod sim_data;
mod sim_helper;
mod simulations;
mod simulation;
mod sim_behavior;

use crate::sim_behavior::SimBehavior;
use crate::sim_data::SimData;
use crate::sim_helper::*;
use crate::simulations::*;
use crate::simulation::Simulation;

use crate::fade::fade_simulation;
use crate::cool1::cool1_simulation;
use crate::conways::conways_simulation;



fn main() {

    let mut simulation = conways_simulation(); // Change this to switch between simulations

    let mut window = Window::new(
        "Simple Blue Box",
        simulation.width,
        simulation.height,
        WindowOptions {
            resize: true,
            scale: simulation.scale,
            scale_mode: minifb::ScaleMode::AspectRatioStretch,
            ..WindowOptions::default()
        },
    )
    .unwrap();

    simulation.data.set_grids(&init_random_bw(simulation.width, simulation.height));

    window.set_target_fps(simulation.fps);

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        simulation.step();

        window
            .update_with_buffer(&simulation.output().as_slice(), simulation.width, simulation.height)
            .unwrap();
    }
}
