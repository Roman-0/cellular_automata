use std::mem;

pub struct SimData<E = ()>
where
    E: Default,
{
    pub width: usize,
    pub height: usize,
    pub grid: Vec<u32>,
    pub next_grid: Vec<u32>,

    pub extra: E,
}

impl<E> SimData<E>
where
    E: Default,
{
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
            return u32::default();
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
        if new_grid.len() != self.width() * self.height() {
            panic!("New grid size does not match SimData dimensions");
        }
        self.grid.copy_from_slice(new_grid);
        self.next_grid.copy_from_slice(new_grid);
    }
}

    impl<E : Default> SimData<E> {
    pub fn for_each_neighbor_4<F>(&self, idx: usize, looping: bool, mut f: F)
    where
        F: FnMut(u32),
    {
        if idx >= self.width * self.height {
            panic!("Index too large (for_each_neighbor_4)");
        }
        let w = self.width;
        let h = self.height;
        let x = idx % w;
        let y = idx / w;

        // Relative steps for: Left, Right, Up, Down
        let offsets = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        for (dx, dy) in offsets {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if looping {
                // Modulo wrap around logic for Toroidal grids
                let tx = ((nx % w as isize + w as isize) % w as isize) as usize;
                let ty = ((ny % h as isize + h as isize) % h as isize) as usize;
                f(self.grid[ty * w + tx]);
            } else {
                // Standard clamping boundary (ignore offscreen elements)
                if nx >= 0 && nx < w as isize && ny >= 0 && ny < h as isize {
                    f(self.grid[ny as usize * w + nx as usize]);
                }
            }
        }
    }

    pub fn for_each_neighbor_8<F>(&self, idx: usize, looping: bool, mut f: F)
    where
        F: FnMut(u32),
    {
        if idx >= self.width * self.height {
            panic!("Index too large (for_each_neighbor_8)");
        }
        let w = self.width;
        let h = self.height;
        let x = idx % w;
        let y = idx / w;

        // Relative steps for all 8 directions
        let offsets = [
            (-1, -1), (0, -1), (1, -1),
            (-1,  0),          (1,  0),
            (-1,  1), (0,  1), (1,  1),
        ];

        for (dx, dy) in offsets {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if looping {
                let tx = ((nx % w as isize + w as isize) % w as isize) as usize;
                let ty = ((ny % h as isize + h as isize) % h as isize) as usize;
                f(self.grid[ty * w + tx]);
            } else {
                if nx >= 0 && nx < w as isize && ny >= 0 && ny < h as isize {
                    f(self.grid[ny as usize * w + nx as usize]);
                }
            }
        }
    }
}


impl<E: Default> SimData<E> {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width: width,
            height: height,
            grid: vec![0; width * height],
            next_grid: vec![0; width * height],

            extra: E::default(),
        }
    }
    pub fn new_with_extra(width: usize, height: usize, extra: E) -> Self {
        Self {
            width: width,
            height: height,
            grid: vec![0; width * height],
            next_grid: vec![0; width * height],

            extra,
        }
    }
}
