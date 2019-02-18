/*!
Externals.
!*/

#![cfg(windows)]

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

pub mod ptr;
pub mod error;
pub mod process;
pub mod vm;
pub mod module;
pub mod thread;
pub mod window;
pub mod wndclass;
pub mod hook;
pub mod input;
pub mod control;
pub mod snap;
pub mod system;

pub mod prelude;
