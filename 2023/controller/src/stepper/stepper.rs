#[derive(Debug, Clone)]
pub enum Direction {
  Forward,
  Backward,
}

#[derive(Debug, Clone)]
pub enum Mode {
  Full,
  Half,
  Quarter,
  Eighth,
  Sixteenth,
}

pub trait Stepper {
  fn step(&mut self, mode: Mode, direction: Direction);
}
