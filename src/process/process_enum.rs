use std::{fmt, mem};
use std::ffi::{OsString};
use std::os::windows::ffi::{OsStringExt};
use crate::winapi::*;
use crate::util::from_wchar_buf;
use crate::error::ErrorCode;
use crate::Result;
use super::ProcessId;

//----------------------------------------------------------------

/// Process enumeration.
///
/// Uses the Toolhelp32 snapshot API.
#[derive(Debug)]
pub struct EnumProcess {
	handle: HANDLE,
	next: bool,
}
impl EnumProcess {
	/// Iterate over the running processes.
	pub fn create() -> Result<EnumProcess> {
		let handle = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };
		if handle == INVALID_HANDLE_VALUE {
			Err(ErrorCode::last())
		}
		else {
			Ok(EnumProcess { handle, next: false })
		}
	}
}
impl Iterator for EnumProcess {
	type Item = ProcessEntry;
	fn next(&mut self) -> Option<ProcessEntry> {
		unsafe {
			let mut entry: ProcessEntry = mem::zeroed();
			entry.0.dwSize = mem::size_of::<PROCESSENTRY32W>() as DWORD;
			let result = if self.next {
				Process32NextW(self.handle, &mut entry.0)
			}
			else {
				self.next = true;
				Process32FirstW(self.handle, &mut entry.0)
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
impl Drop for EnumProcess {
	fn drop(&mut self) {
		unsafe { CloseHandle(self.handle); }
	}
}

//----------------------------------------------------------------

/// Process entry.
///
/// See [PROCESSENTRY32](https://msdn.microsoft.com/en-us/library/windows/desktop/ms684839.aspx) for more information.
#[derive(Clone)]
pub struct ProcessEntry(PROCESSENTRY32W);
impl_inner!(ProcessEntry: PROCESSENTRY32W);
impl ProcessEntry {
	/// The process identifier.
	pub fn process_id(&self) -> ProcessId {
		ProcessId(self.0.th32ProcessID)
	}
	/// The identifier of the process that created this process (its parent process).
	pub fn parent_id(&self) -> ProcessId {
		ProcessId(self.0.th32ParentProcessID)
	}
	/// The number of execution threads started by the process.
	pub fn thread_count(&self) -> DWORD {
		self.0.cntThreads
	}
	/// The base priority of any threads created by this process.
	pub fn thread_base_priority(&self) -> LONG {
		self.0.pcPriClassBase
	}
	/// The name of the executable file for the process.
	pub fn exe_file_wide(&self) -> &[u16] {
		from_wchar_buf(&self.0.szExeFile)
	}
	/// The name of the executable file for the process.
	pub fn exe_file(&self) -> OsString {
		OsString::from_wide(self.exe_file_wide())
	}
}
impl fmt::Debug for ProcessEntry {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("ProcessEntry")
			.field("dwSize", &self.0.dwSize)
			//.field("cntUsage", &self.0.cntUsage)
			.field("th32ProcessID", &self.process_id())
			//.field("th32DefaultHeapID", &self.0.th32DefaultHeapID)
			//.field("th32ModuleID", &self.0.th32ModuleID)
			.field("cntThreads", &self.0.cntThreads)
			.field("th32ParentProcessID", &self.parent_id())
			.field("pcPriClassBase", &self.0.pcPriClassBase)
			//.field("dwFlags", &self.0.dwFlags)
			.field("szExeFile", &self.exe_file())
			.finish()
	}
}
