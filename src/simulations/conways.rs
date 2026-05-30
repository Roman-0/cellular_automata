use crate::sim_behavior::SimBehavior;
use crate::sim_data::*;
use libm::exp;
use crate::sim_helper::*;
use crate::simulation::*;
use minifb::Scale;
pub struct conways;

impl SimBehavior<()> for conways {
    fn step(&mut self, data: &mut SimData<()>) {
        let total_cells = data.width * data.height;
        for i in 0..total_cells {
            let mut count = 0;

            // Gather all existing neighbors on the screen
            data.for_each_neighbor_8(i, true, |neighbor_value| {
                if neighbor_value == ALIVE {
                    count += 1;
                }
            });
            data.next_grid[i] = match count{
                3 => ALIVE,
                2 => data.grid[i],
                _ => DEAD,
            };
        }
        data.update_grid();
    }
}

pub fn conways_simulation() -> Simulation<(), conways> {
    Simulation::new(conways, SimData::new(320, 240), 320, 240, 60, true, Scale::X4)
}