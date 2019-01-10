/*!
System APIs.
!*/

#[cfg(feature = "ntdll")]
mod system_modules;
mod time;

#[cfg(feature = "ntdll")]
pub use self::system_modules::*;
pub use self::time::*;
