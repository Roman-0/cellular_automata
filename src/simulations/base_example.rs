
use crate::sim_behavior::SimBehavior;
use crate::sim_data::*;


pub struct BehaviorBase;


impl SimBehavior<()> for BehaviorBase{
  fn step(&mut self, data: &mut SimData<()>){
    for i in 0..data.width() * data.height() {
        data.next_grid[i] = data.grid[i].wrapping_add(100);
    }
    data.update_grid();
  }
}

