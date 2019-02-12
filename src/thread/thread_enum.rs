use std::{mem, fmt};
use crate::winapi::*;
use crate::process::ProcessId;
use crate::thread::ThreadId;
use crate::error::ErrorCode;
use crate::{Result, FromInner};

//----------------------------------------------------------------

/// Thread enumeration.
///
/// Uses the Toolhelp32 snapshot API.
#[derive(Debug)]
pub struct EnumThreads(HANDLE, bool);
impl EnumThreads {
	/// Iterate over all running threads.
	pub fn create() -> Result<EnumThreads> {
		let handle = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0) };
		if handle == INVALID_HANDLE_VALUE {
			Err(ErrorCode::last())
		}
		else {
			Ok(EnumThreads(handle, false))
		}
	}
}
impl Iterator for EnumThreads {
	type Item = ThreadEntry;
	fn next(&mut self) -> Option<ThreadEntry> {
		unsafe {
			let mut entry: ThreadEntry = mem::uninitialized();
			entry.0.dwSize = mem::size_of::<THREADENTRY32>() as DWORD;
			let result = if self.1 {
				Thread32Next(self.0, &mut entry.0)
			}
			else {
				self.1 = true;
				Thread32First(self.0, &mut entry.0)
			};
			if result != FALSE {
				Some(entry)
			}
			else {
				None
			}
		}
	}
}
impl Drop for EnumThreads {
	fn drop(&mut self) {
		unsafe { CloseHandle(self.0); }
	}
}

//----------------------------------------------------------------

/// Thread entry.
///
/// See [THREADENTRY32](https://msdn.microsoft.com/en-us/library/windows/desktop/ms686735.aspx) for more information.
pub struct ThreadEntry(THREADENTRY32);
impl_inner!(ThreadEntry: THREADENTRY32);
impl ThreadEntry {
	/// The thread identifier.
	pub fn thread_id(&self) -> ThreadId {
		unsafe { ThreadId::from_inner(self.0.th32ThreadID) }
	}
	/// The identifier of the process that created the thread.
	pub fn process_id(&self) -> ProcessId {
		unsafe { ProcessId::from_inner(self.0.th32OwnerProcessID) }
	}
	/// The kernel base priority level assigned to the thread.
	pub fn base_priority(&self) -> LONG {
		self.0.tpBasePri
	}
}
impl fmt::Debug for ThreadEntry {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("ThreadEntry")
			.field("dwSize", &self.0.dwSize)
			//.field("cntUsage", &self.0.cntUsage)
			.field("th32ThreadID", &self.0.th32ThreadID)
			.field("th32OwnerProcessID", &self.0.th32OwnerProcessID)
			.field("tpBasePri", &self.0.tpBasePri)
			//.field("tpDeltaPri", &self.0.tpDeltaPri)
			//.field("dwFlags", &self.0.dwFlags)
			.finish()
	}
}
