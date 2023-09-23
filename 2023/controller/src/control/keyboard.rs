use async_trait::async_trait;
use crossterm::{
  event::{self, KeyCode, KeyEvent},
  terminal::{disable_raw_mode, enable_raw_mode},
};
use log::{error, info};
use tokio::time::{sleep, Duration};

use super::Control;
use crate::stepper::{Direction, Mode, Stepper};

pub struct Keyboard<S: Stepper + Send + Sync> {
  stepper: S,
  mode: Mode,
}

impl<S> Keyboard<S>
where
  S: Stepper + Send + Sync,
{
  pub fn new(stepper: S) -> Self {
    Keyboard {
      stepper,
      mode: Mode::Full,
    }
  }

  pub fn previous_mode(&mut self) {
    self.mode = match self.mode {
      Mode::Full => Mode::Full,
      Mode::Half => Mode::Quarter,
      Mode::Quarter => Mode::Eighth,
      Mode::Eighth => Mode::Sixteenth,
      Mode::Sixteenth => Mode::Sixteenth,
    }
  }

  pub fn next_mode(&mut self) {
    self.mode = match self.mode {
      Mode::Full => Mode::Full,
      Mode::Half => Mode::Full,
      Mode::Quarter => Mode::Half,
      Mode::Eighth => Mode::Quarter,
      Mode::Sixteenth => Mode::Sixteenth,
    }
  }
}

#[async_trait]
impl<S> Control for Keyboard<S>
where
  S: Stepper + Send + Sync,
{
  async fn run(&mut self) {
    loop {
      if let Err(_) = enable_raw_mode() {
        panic!("failed to enable raw mode");
      }

      if event::poll(Duration::from_millis(100)).unwrap() {
        if let event::Event::Key(KeyEvent {
          code, ..
        }) = event::read().unwrap()
        {
          // disable raw mode to avoid logging issues
          if let Err(_) = disable_raw_mode() {
            error!("failed to disable raw mode");
          }

          match code {
            KeyCode::Esc => {
              info!("exiting");
              break;
            }
            KeyCode::Up => {
              info!("up");
              self.next_mode();
            }
            KeyCode::Down => {
              info!("down");
              self.previous_mode();
            }
            KeyCode::Left => {
              info!("left");
              self.stepper.step(self.mode.clone(), Direction::Backward);
            }
            KeyCode::Right => {
              info!("right");
              self.stepper.step(self.mode.clone(), Direction::Forward);
            }
            _ => {
              info!("unhandled key");
            }
          }
        }
      }
      sleep(Duration::from_millis(50)).await;
    }

    // Disable raw mode before exiting
    if let Err(_) = disable_raw_mode() {
      error!("failed to disable raw mode");
    }
  }
}
