/*!
Externals.
*/

extern crate winapi;

#[cfg(all(windows, feature = "ntapi"))]
extern crate ntdll;

mod util;
#[macro_use]
mod inner;
pub use inner::*;

#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use windows::*;
