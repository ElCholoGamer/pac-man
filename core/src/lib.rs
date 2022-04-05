mod emulator;
mod cpu;
mod memory;
mod error;

pub use error::*;
pub use emulator::*;
pub use cpu::{CPU, Event as CPUEvent};
pub use memory::*;

#[macro_export]
macro_rules! concat_u16 {
    ($hi:expr,$lo:expr) => {
        (($hi as u16) << 8) | ($lo as u16)
    };
}