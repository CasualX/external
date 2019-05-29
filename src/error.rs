/*!
Windows error codes.
!*/

use std::{fmt, error};
use crate::winapi::*;

/// Windows error code.
///
/// See [System Error Codes](https://msdn.microsoft.com/en-us/library/windows/desktop/ms681381.aspx) for more information.
#[derive(Copy, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ErrorCode(DWORD);
impl_inner!(ErrorCode: safe DWORD);
impl ErrorCode {
	pub const SUCCESS: ErrorCode = ErrorCode(0);
}
impl ErrorCode {
	/// Returns true if this is the success error code.
	pub const fn is_success(self) -> bool {
		self.0 == 0
	}
	/// Gets the last error code.
	///
	/// See [GetLastError function](https://msdn.microsoft.com/en-us/library/windows/desktop/ms679360.aspx) for more information.
	pub fn last() -> ErrorCode {
		ErrorCode(unsafe { GetLastError() })
	}
}
impl fmt::Display for ErrorCode {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:#X}", self.0)
	}
}
impl fmt::Debug for ErrorCode {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "ErrorCode({:#X})", self.0)
	}
}
impl error::Error for ErrorCode {
	fn description(&self) -> &str {
		"system error code"
	}
}
