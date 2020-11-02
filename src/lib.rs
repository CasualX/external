/*!
Externals.
!*/

#![cfg(windows)]

#![cfg_attr(feature = "nightly", feature(asm))]

mod util;
pub use self::util::*;

#[macro_use]
mod inner;
pub use self::inner::*;

macro_rules! wide_str {
    ($($c:tt)+) => {
        [$($c as u16,)+]
    }
}

mod winapi;

pub type Result<T> = std::result::Result<T, error::ErrorCode>;

pub mod error;
pub mod process;
pub mod module;
pub mod thread;
pub mod window;
pub mod wndclass;
pub mod hook;
pub mod vk;
pub mod memory;
pub mod mouse;
pub mod control;
pub mod snap;
pub mod system;

pub mod prelude;
