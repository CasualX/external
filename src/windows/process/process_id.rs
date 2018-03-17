use winapi::shared::minwindef::DWORD;

/// Wraps a process identifier.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ProcessId(pub(super) DWORD);
impl_inner!(ProcessId: DWORD);
