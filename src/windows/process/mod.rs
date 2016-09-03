/*!
Process handles.
*/

mod processes;
mod rights;
mod handle;

pub use self::processes::*;
pub use self::rights::ProcessRights;
pub use self::handle::Process;

//----------------------------------------------------------------

use winapi::DWORD;

/// Wraps a process identifier.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ProcessId(DWORD);
impl_inner!(ProcessId: DWORD);
