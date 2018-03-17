/*!
Externals.
*/

extern crate winapi;

mod util;
#[macro_use]
mod inner;
pub use inner::*;

#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use windows::*;
