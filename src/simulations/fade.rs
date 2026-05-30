
use crate::sim_behavior::SimBehavior;
use crate::sim_data::*;
use crate::simulation::*;
use minifb::Scale;

pub struct FadeBehavior;

impl SimBehavior<()> for FadeBehavior {
    fn step(&mut self, data: &mut SimData<()>) {
        let w = data.width();
        let h = data.height();
        let total_cells = w * h;

        for i in 0..total_cells {
            // Start with the current cell's value
            let mut count = 1;
            let mut sum: u64 = data.grid[i] as u64;

            // Gather all existing neighbors on the screen
            data.for_each_neighbor_8(i, false, |neighbor_value| {
                count += 1;
                sum += neighbor_value as u64;
            });
            
            // Calculate the blurred average
            data.next_grid[i] = (sum / count) as u32;
        }

       
        data.update_grid(); 
    }
}

pub fn fade_simulation() -> Simulation<(), FadeBehavior> {
    Simulation::new(FadeBehavior, SimData::new(640, 480), 640, 480, 60, true, Scale::X2)
}