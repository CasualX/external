/*!
Error codes.
*/

use std::{fmt, error};

use winapi::um::errhandlingapi::{GetLastError};
use winapi::shared::minwindef::{DWORD};

/// Windows error code.
///
/// See [System Error Codes](https://msdn.microsoft.com/en-us/library/windows/desktop/ms681381.aspx) for more information.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ErrorCode(DWORD);
impl_inner!(ErrorCode: DWORD);
impl ErrorCode {
	/// Get the last error code.
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

pub const ERROR_SUCCESS: ErrorCode = ErrorCode(0);
