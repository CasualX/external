/*!
*/

macro_rules! wide_str {
    ($($c:tt)+) => {
        [$($c as u16,)+]
    }
}

pub mod ptr;
pub mod error;
pub mod process;
pub mod vm;
pub mod module;
pub mod thread;
// pub mod thread_token;
pub mod window;
pub mod wndclass;
pub mod hook;
pub mod input;
pub mod control;
pub mod snap;

pub type Result<T> = ::std::result::Result<T, error::ErrorCode>;

pub mod flat {
	pub use super::Result;
	pub use super::ptr::*;
	pub use super::error::*;
	pub use super::process::*;
	pub use super::vm::*;
	pub use super::module::*;
	pub use super::thread::*;
	// pub use super::thread_token::*;
	pub use super::window::*;
	pub use super::wndclass::*;
	pub use super::hook::*;
	pub use super::input::*;
	pub use super::control::*;
	pub use ::{AsInner, AsInnerMut, FromInner, IntoInner};
}
