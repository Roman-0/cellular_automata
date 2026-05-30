use std::mem;

pub struct SimData <E = ()>{
 pub width: usize,
 pub height: usize,
 pub grid: Vec<u32>,
 pub next_grid: Vec<u32>,

 pub extra: E,
}

impl<E> SimData<E> {
    pub fn output(&self) -> &Vec<u32> {
        &self.grid
    }

    pub fn width(&self) -> usize {
        self.width as usize
    }

    pub fn height(&self) -> usize {
        self.height as usize
    }

    pub fn get_cell(&self, x: usize, y: usize) -> u32 {
        if x >= self.width() || y >= self.height() {
            return 0;
        }
        self.grid[y * self.width() + x]
    }

    pub fn swap_grids(&mut self) {
        std::mem::swap(&mut self.grid, &mut self.next_grid);
    }

    pub fn update_grid(&mut self) {
        self.grid.copy_from_slice(&self.next_grid);
    }

    pub fn set_grid(&mut self, new_grid: &[u32]) {
        if new_grid.len() != self.width() * self.height() {
            panic!("New grid size does not match SimData dimensions");
        }
        self.grid.copy_from_slice(new_grid);
    }

    pub fn set_next_grid(&mut self, new_grid: &[u32]) {
        if new_grid.len() != self.width() * self.height() {
            panic!("New grid size does not match SimData dimensions");
        }
        self.next_grid.copy_from_slice(new_grid);
    }

    pub fn set_grids(&mut self, new_grid: &[u32]) {
        if new_grid.len() != self.width() * self.height(){
            panic!("New grid size does not match SimData dimensions");
        }
        self.grid.copy_from_slice(new_grid);
        self.next_grid.copy_from_slice(new_grid);
    }
}




impl<E: Default> SimData<E> {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width: width,
            height: height,
            grid: vec![0x00_00_00_00; width * height],
            next_grid: vec![0x00_00_00_00; width * height],

            extra: E::default(),
        }
    }
  pub fn new_with_extra(width: usize, height: usize, extra: E) -> Self {
        Self {
            width: width,
            height: height,
            grid: vec![0x00_00_00_00; width * height],
            next_grid: vec![0x00_00_00_00; width * height],

            extra,
        }
    }
}
