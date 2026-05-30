
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