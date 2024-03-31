//! # Device
//!
//! Devices implementations. NOT ARCH-INDEPENDENT.
//! ---
//! Change log:
//!   - 2024/03/14: File created.

#[macro_use]
pub mod console;
pub mod timer;
pub mod virtio;

pub use console::{Console, Write as ConsoleWrite};
use crate::do_init;

pub fn init() {
    do_init!(
        console,
        timer,
        virtio
    );
}