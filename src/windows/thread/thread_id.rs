use winapi::shared::minwindef::{DWORD};

/// Wraps a thread identifier.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ThreadId(pub(super) DWORD);
impl_inner!(ThreadId: DWORD);
