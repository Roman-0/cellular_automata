use crate::sim_behavior::SimBehavior;
use crate::sim_data::*;
use libm::exp;

pub struct conways;

enum Status{
    Alive,
    Dead
}



fn conwayHelper()

impl SimBehavior<()> for conways {
    fn step(&mut self, data: &mut SimData<()>) {
        for i in 0..data.width() * data.height() {
            data.next_grid[i] =
                data.grid[i]
        }
        data.update_grid();
    }
}
