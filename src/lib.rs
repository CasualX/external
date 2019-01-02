/*!
Externals.
*/

mod util;

#[macro_use]
mod inner;
pub use crate::inner::*;

#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use crate::windows::*;
