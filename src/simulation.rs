use crate::sim_behavior::SimBehavior;
use crate::sim_data::SimData;
use crate::sim_helper::*;
use minifb::Scale;

pub struct Simulation<E: Default, B: SimBehavior<E>> {
    pub data: SimData<E>,
    pub behavior: B,
    pub width: usize,
    pub height: usize,
    pub fps: usize,
    pub looping: bool,
    pub scale: Scale,
}

impl<E: Default, B: SimBehavior<E>> Simulation<E, B> {
    pub fn new(behavior: B, data: SimData<E>, width: usize, height: usize, fps: usize, looping: bool, scale: Scale) -> Self {
        Self { behavior, data, width, height, fps, looping, scale }
    }

    pub fn step(&mut self) {
        self.behavior.step(&mut self.data);
    }

    pub fn output(&self) -> &Vec<u32> {
        self.data.output()
    }
}
