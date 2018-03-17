use std::fmt;

use winapi::shared::minwindef::{DWORD};

/// Wraps a thread identifier.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ThreadId(pub(super) DWORD);
impl_inner!(ThreadId: DWORD);

// Custom Debug implementation to disable pretty formatting
impl fmt::Debug for ThreadId {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "ThreadId({})", self.0)
	}
}
