#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
mod raspi;

#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
pub use raspi::Raspi as StepperImpl;

#[cfg(not(all(target_os = "linux", target_arch = "aarch64")))]
mod noop;

#[cfg(not(all(target_os = "linux", target_arch = "aarch64")))]
pub use self::noop::NoOp as StepperImpl;

mod stepper;
pub use stepper::{Direction, Mode, Stepper};
