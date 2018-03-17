/*!
System.
*/

#[cfg(feature = "ntapi")]
mod system_modules;

#[cfg(feature = "ntapi")]
pub use self::system_modules::*;
