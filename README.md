# Provides a Framework to Implement your own simulations.
# Usage
``` rust
// (0) Standard library imports
use crate::sim_behavior::SimBehavior;
use crate::sim_data::*;
use crate::simulation::*;
use minifb::Scale;


// (1) Create a Blank Struct
pub struct BehaviorExample;

// (2) Implement the SimBehavior trait for the struct
impl SimBehavior<()> for BehaviorExample {
    fn step(&mut self, data: &mut SimData<()>) {
        let total_cells = data.width() * data.height();
        for i in 0..total_cells {
            data.next_grid[i] =
                data.grid[i].wrapping_add(1) as u32);
        }
        data.update_grid();
    }
}

// (3) Create a function to initialize the simulation
pub fn example_simulation() -> Simulation<(), BehaviorExample> {
    Simulation::new(BehaviorExample, SimData::new(640, 480), 640, 480, 60, true, Scale::X1)
}
```


# Examples
Cool1.rs

<img width="480" height="480" alt="image" src="https://github.com/user-attachments/assets/d6c26811-4dd1-4396-9e60-c97f20faba62" />


conways.rs

<img width="480" height="478" alt="image" src="https://github.com/user-attachments/assets/62787d43-8b9e-4a97-908c-1eba4f284592" />

