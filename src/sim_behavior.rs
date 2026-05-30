use crate::sim_data::SimData;

pub trait SimBehavior<T: Default + Copy + Into<u32>, E: Default> {
    fn step(&mut self, data: &mut SimData<T, E>);
}

pub struct Simulation<T: Default + Copy + Into<u32>, E: Default, B: SimBehavior<T, E>> {
    pub data: SimData<T, E>,
    pub behavior: B,
}

impl<T: Default + Copy + Into<u32>, E: Default, B: SimBehavior<T, E>> Simulation<T, E, B> {
    pub fn new(behavior: B, data: SimData<T, E>) -> Self {
        Self { behavior, data }
    }

    pub fn step(&mut self) {
        self.behavior.step(&mut self.data);
    }

    pub fn output(&self) -> Vec<u32> {
        self.data.output()
    }
}
