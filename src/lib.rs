/*!
Externals.
!*/

mod util;

#[macro_use]
mod inner;
pub use self::inner::*;

#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use self::windows::*;
