// Reexport winapi in a single flat namespace

pub use winapi::um::consoleapi::*;
pub use winapi::um::errhandlingapi::*;
pub use winapi::um::handleapi::*;
pub use winapi::um::memoryapi::*;
pub use winapi::um::processthreadsapi::*;
pub use winapi::um::profileapi::*;
pub use winapi::um::psapi::*;
pub use winapi::um::synchapi::*;
pub use winapi::um::tlhelp32::*;
pub use winapi::um::winbase::*;
pub use winapi::um::wincon::*;
pub use winapi::um::wingdi::*;
pub use winapi::um::winnt::*;
pub use winapi::um::winuser::*;
pub use winapi::shared::basetsd::*;
pub use winapi::shared::minwindef::*;
// pub use winapi::shared::ntdef::*;
pub use winapi::shared::ntdef::UNICODE_STRING;
pub use winapi::shared::windef::*;
pub use winapi::shared::winerror::*;
pub use winapi::ctypes::*;

pub use ntapi::ntexapi::*;
pub use ntapi::ntldr::*;
