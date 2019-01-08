use std::fmt;

use winapi::shared::minwindef::DWORD;

/// Wraps a process identifier.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ProcessId(pub(super) DWORD);
impl_inner!(ProcessId: DWORD);

// Custom Debug and Display implementation to disable pretty formatting
impl fmt::Debug for ProcessId {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "ProcessId({})", self.0)
	}
}
impl fmt::Display for ProcessId {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.0.fmt(f)
	}
}
