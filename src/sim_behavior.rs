use crate::sim_data::SimData;

pub trait SimBehavior<E: Default> {
    fn step(&mut self, data: &mut SimData<E>);
}

