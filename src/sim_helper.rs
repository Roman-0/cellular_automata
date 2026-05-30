use rand::{random, random_bool};

pub const DEAD: u32 = 0xFF_FF_FF_FF;
pub const ALIVE: u32 = 0x00_00_00_00;

pub fn init_random_bw(width: usize, height: usize) -> Vec<u32> {
    let mut grid = Vec::with_capacity(width * height);
    for _ in 0..width * height {
        grid.push(if random_bool(0.5) {
            0xFF_FF_FF_FF
        } else {
            0x00_00_00_00
        });
    }
    grid
}

pub fn init_random_color(width: usize, height: usize) -> Vec<u32> {
    let mut grid = Vec::with_capacity(width * height);
    for _ in 0..width * height {
        grid.push(rand::random::<u32>());
    }
    grid
}
