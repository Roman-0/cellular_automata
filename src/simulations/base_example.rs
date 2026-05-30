use crate::sim_behavior::SimBehavior;
use crate::sim_data::*;
use crate::simulation::*;
use libm::exp;
use minifb::Scale;

pub struct BehaviorExample;

impl SimBehavior<()> for BehaviorExample {
    fn step(&mut self, data: &mut SimData<()>) {
        let total_cells = data.width() * data.height();
        for i in 0..total_cells {
            data.next_grid[i] =
                data.grid[i].wrapping_add(1) as u32);
        }
        data.update_grid();
    }
}

pub fn example_simulation() -> Simulation<(), BehaviorExample> {
    Simulation::new(BehaviorExample, SimData::new(640, 480), 640, 480, 60, true, Scale::X1)
}