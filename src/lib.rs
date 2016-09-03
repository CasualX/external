/*!
Externals.
*/

extern crate winapi;
extern crate kernel32;
extern crate advapi32;
extern crate user32;
extern crate gdi32;

mod util;
#[macro_use]
mod inner;
pub use inner::*;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::*;
