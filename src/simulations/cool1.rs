use crate::sim_behavior::SimBehavior;
use crate::sim_data::*;
use crate::simulation::*;
use minifb::Scale;
pub fn for_each_neighbor_cool1_8<E: Default, F>(data: &SimData<E>, idx: usize, looping: bool, mut f: F)
    where
        F: FnMut(u32),
{
        if idx >= data.width * data.height {
            panic!("Index too large (Get_Neighbors_8)");
        }
        let x = idx % data.width;
        let y = idx / data.width;

        let y0 = y == 0;
        let yh = y == data.height - 1;

        if x != 0 {
            f(data.grid[idx - 1]);
            if !y0 {
                f(data.grid[idx - data.width - 1]);
            }
            if !yh {
                f(data.grid[idx + data.width - 1]);
            }
        }
        if x != data.width - 1 {
            f(data.grid[idx + 1]);
            if !y0 {
                f(data.grid[idx - data.width + 1]);
            }
            if !yh {
                f(data.grid[idx + data.width + 1]);
            }
        }
        if !y0 {
            f(data.grid[idx - data.width]);
        }
        if !yh {
            f(data.grid[idx + data.width]);
        }

        if looping {
            if x == 0 {
                //left
                f(data.grid[idx + data.width - 1]);
                if y0 {
                    f(data.grid[data.width * data.height - 1]);
                } //top left
                if yh {
                    f(data.grid[data.width - 1]);
                } //bottom left
            }
            if x == data.width - 1 {
                //right
                f(data.grid[idx - data.width - 1]);
                if y0 {
                    f(data.grid[data.width * data.height - data.width]);
                } //top right
                if yh {
                    f(data.grid[data.width - 1]);
                } //top right
            }
            if y0 {
                //top
                f(data.grid[idx + data.width * (data.height - 1)]);
            }
            if yh {
                //bottom
                f(data.grid[idx % data.width]);
            }
        }
}



pub struct cool1Behavior;

impl SimBehavior<()> for cool1Behavior {
    fn step(&mut self, data: &mut SimData<()>) {
        let size = data.width * data.height;
        for i in 0..size {
            let mut count = 1;
            let mut sum: u64 = data.grid[i] as u64;

        // Scan neighbors without allocations or manual Option checking!
            for_each_neighbor_cool1_8(data, i, false, |neighbor_value| {
                    count += 1;
                    sum += neighbor_value as u64;
            });
            
            data.next_grid[i] = (sum / count) as u32;
        }
        data.update_grid();
    }
}

pub fn cool1_simulation() -> Simulation<(), cool1Behavior> {
    Simulation::new(cool1Behavior, SimData::new(640, 480), 640, 480, 800, true, Scale::X2)
}