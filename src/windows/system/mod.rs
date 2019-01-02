/*!
System APIs.
*/

#[cfg(feature = "ntdll")]
mod system_modules;

#[cfg(feature = "ntdll")]
pub use self::system_modules::*;
