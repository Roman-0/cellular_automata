

use crate::sim_data::{SimData};

pub trait SimBehavior<E>{
  fn step(&mut self, data: &mut SimData<E>);
}


pub struct Simulation<B: SimBehavior<E>, E> {
  pub data: SimData<E>,
  pub behavior: B,
}

impl<B: SimBehavior<E>, E> Simulation<B, E>{
  pub fn new(behavior: B, data: SimData<E>) -> Self{
    Self{behavior, data}
  }

  pub fn step(&mut self){
    self.behavior.step(&mut self.data);
  }

  pub fn output(&self) -> &Vec<u32>{
    self.data.output()
  }

  
}