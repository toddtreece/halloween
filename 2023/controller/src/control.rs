#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
mod keyboard;
//mod raspi;

#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
pub use self::keyboard::Keyboard as ControlImpl;
//pub use raspi::Raspi as ControlImpl;

#[cfg(not(all(target_os = "linux", target_arch = "aarch64")))]
mod keyboard;

#[cfg(not(all(target_os = "linux", target_arch = "aarch64")))]
pub use self::keyboard::Keyboard as ControlImpl;

mod control;
pub use control::Control;
