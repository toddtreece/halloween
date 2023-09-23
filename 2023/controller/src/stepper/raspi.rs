use super::{Direction, Mode, Stepper};
use rppal::gpio::{Gpio, OutputPin};
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug)]
pub struct Raspi {
  ms1: OutputPin,
  ms2: OutputPin,
  ms3: OutputPin,
  en: OutputPin,
  dir: OutputPin,
  step: OutputPin,
}

impl Raspi {
  pub fn new() -> Self {
    let gpio = Gpio::new().unwrap();
    let ms1 = gpio.get(5).unwrap().into_output();
    let ms2 = gpio.get(6).unwrap().into_output();
    let ms3 = gpio.get(13).unwrap().into_output();
    let en = gpio.get(21).unwrap().into_output();
    let dir = gpio.get(26).unwrap().into_output();
    let step = gpio.get(19).unwrap().into_output();

    Raspi {
      ms1,
      ms2,
      ms3,
      en,
      dir,
      step,
    }
  }

  pub fn set_mode(&mut self, mode: Mode) {
    match mode {
      Mode::Full => {
        self.ms1.set_low();
        self.ms2.set_low();
        self.ms3.set_low();
      }
      Mode::Half => {
        self.ms1.set_high();
        self.ms2.set_low();
        self.ms3.set_low();
      }
      Mode::Quarter => {
        self.ms1.set_low();
        self.ms2.set_high();
        self.ms3.set_low();
      }
      Mode::Eighth => {
        self.ms1.set_high();
        self.ms2.set_high();
        self.ms3.set_low();
      }
      Mode::Sixteenth => {
        self.ms1.set_high();
        self.ms2.set_high();
        self.ms3.set_high();
      }
    }
  }

  pub fn set_direction(&mut self, direction: Direction) {
    match direction {
      Direction::Forward => {
        self.dir.set_high();
      }
      Direction::Backward => {
        self.dir.set_low();
      }
    }
  }

  pub fn enable(&mut self) {
    self.en.set_low();
  }
}

impl Stepper for Raspi {
  fn step(&mut self, mode: Mode, direction: Direction) {
    self.enable();
    self.set_mode(mode);
    self.set_direction(direction);

    self.step.set_high();
    sleep(Duration::from_millis(1));
    self.step.set_low();
    sleep(Duration::from_millis(1));
  }
}
