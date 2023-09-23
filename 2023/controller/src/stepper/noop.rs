use log::info;

use super::{Direction, Mode, Stepper};

#[derive(Debug)]
pub struct NoOp {}

impl NoOp {
  pub fn new() -> Self {
    NoOp {}
  }
}

impl Stepper for NoOp {
  fn step(&mut self, mode: Mode, direction: Direction) {
    info!("Mode: {:?} Direction: {:?}", mode, direction);
  }
}
