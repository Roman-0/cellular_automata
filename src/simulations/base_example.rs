use crate::sim::{SimData, SimBehavior};



pub struct BehaviorBase;

impl SimBehavior<()> for BehaviorBase{
  fn step(&mut self, data: &mut SimData<()>){
    
  }
}

