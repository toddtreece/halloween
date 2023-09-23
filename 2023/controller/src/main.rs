use log::info;

mod control;
mod stepper;

use control::{Control, ControlImpl};
use stepper::StepperImpl;

#[tokio::main]
async fn main() {
  env_logger::init();

  let stepper = StepperImpl::new();
  let mut control = ControlImpl::new(stepper);

  info!("Starting control loop");
  control.run().await;
}
