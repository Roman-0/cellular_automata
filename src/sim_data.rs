use std::mem;

pub struct SimData<T, E = ()>
where
    T: Into<u32> + Default + Copy,
    E: Default,
{
    pub width: usize,
    pub height: usize,
    pub grid: Vec<T>,
    pub next_grid: Vec<T>,

    pub extra: E,
}

impl<T, E> SimData<T, E>
where
    T: Into<u32> + Default + Copy,
    E: Default,
{
    pub fn output(&self) -> Vec<u32> {
        self.grid.iter().map(|x| (*x).into()).collect()
    }

    pub fn width(&self) -> usize {
        self.width as usize
    }

    pub fn height(&self) -> usize {
        self.height as usize
    }

    pub fn get_cell(&self, x: usize, y: usize) -> T {
        if x >= self.width() || y >= self.height() {
            return T::default();
        }
        self.grid[y * self.width() + x]
    }

    pub fn swap_grids(&mut self) {
        std::mem::swap(&mut self.grid, &mut self.next_grid);
    }

    pub fn update_grid(&mut self) {
        self.grid.copy_from_slice(&self.next_grid);
    }

    pub fn set_grid(&mut self, new_grid: &[T]) {
        if new_grid.len() != self.width() * self.height() {
            panic!("New grid size does not match SimData dimensions");
        }
        self.grid.copy_from_slice(new_grid);
    }

    pub fn set_next_grid(&mut self, new_grid: &[T]) {
        if new_grid.len() != self.width() * self.height() {
            panic!("New grid size does not match SimData dimensions");
        }
        self.next_grid.copy_from_slice(new_grid);
    }

    pub fn set_grids(&mut self, new_grid: &[T]) {
        if new_grid.len() != self.width() * self.height() {
            panic!("New grid size does not match SimData dimensions");
        }
        self.grid.copy_from_slice(new_grid);
        self.next_grid.copy_from_slice(new_grid);
    }

    pub fn get_neighbors_4<F>(&self, idx: usize, looping: bool, mut f: F)
    where
        F: FnMut(T),
    {
        if idx >= self.width * self.height {
            panic!("Index too large (Get_Neighbors_4)");
        }
        let x = idx % self.width;
        let y = idx / self.width;

        if x != 0 {
            f(self.grid[idx - 1]);
        }
        if x != self.width - 1 {
            f(self.grid[idx + 1]);
        }
        if y != 0 {
            f(self.grid[idx - self.width]);
        }
        if y != self.height - 1 {
            f(self.grid[idx + self.width]);
        }

        if looping {
            if x == 0 {
                f(self.grid[idx + self.width - 1]);
            }
            if x == self.width - 1 {
                f(self.grid[idx - self.width - 1]);
            }
            if y == 0 {
                f(self.grid[idx + self.width * (self.height - 1)]);
            }
            if y == self.height - 1 {
                f(self.grid[idx % self.width]);
            }
        }
    }

    pub fn get_neighbors_8<F>(&self, idx: usize, looping: bool, mut f: F)
    where
        F: FnMut(T),
    {
        if idx >= self.width * self.height {
            panic!("Index too large (Get_Neighbors_8)");
        }
        let x = idx % self.width;
        let y = idx / self.width;

        let y0 = y == 0;
        let yh = y == self.height - 1;

        if x != 0 {
            f(self.grid[idx - 1]);
            if !y0 {
                f(self.grid[idx - self.width - 1]);
            }
            if !yh {
                f(self.grid[idx + self.width - 1]);
            }
        }
        if x != self.width - 1 {
            f(self.grid[idx + 1]);
            if !y0 {
                f(self.grid[idx - self.width + 1]);
            }
            if !yh {
                f(self.grid[idx + self.width + 1]);
            }
        }
        if !y0 {
            f(self.grid[idx - self.width]);
        }
        if !yh {
            f(self.grid[idx + self.width]);
        }

        if looping {
            if x == 0 {
                //left
                f(self.grid[idx + self.width - 1]);
                if y0 {
                    f(self.grid[self.width * self.height - 1]);
                } //top left
                if yh {
                    f(self.grid[self.width - 1]);
                } //bottom left
            }
            if x == self.width - 1 {
                //right
                f(self.grid[idx - self.width - 1]);
                if y0 {
                    f(self.grid[self.width * self.height - self.width]);
                } //top right
                if yh {
                    f(self.grid[self.width - 1]);
                } //top right
            }
            if y0 {
                //top
                f(self.grid[idx + self.width * (self.height - 1)]);
            }
            if yh {
                //bottom
                f(self.grid[idx % self.width]);
            }
        }
    }
}

impl<T: Into<u32> + Default + Copy, E: Default> SimData<T, E> {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width: width,
            height: height,
            grid: vec![T::default(); width * height],
            next_grid: vec![T::default(); width * height],

            extra: E::default(),
        }
    }
    pub fn new_with_extra(width: usize, height: usize, extra: E) -> Self {
        Self {
            width: width,
            height: height,
            grid: vec![T::default(); width * height],
            next_grid: vec![T::default(); width * height],

            extra,
        }
    }
}
