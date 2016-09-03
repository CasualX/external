use std::{fmt, mem};
use std::ffi::{OsString};
use std::os::windows::ffi::{OsStringExt};

use kernel32::{CloseHandle, CreateToolhelp32Snapshot, Process32FirstW, Process32NextW};
use winapi::{FALSE, HANDLE, DWORD, LONG, INVALID_HANDLE_VALUE, TH32CS_SNAPPROCESS, PROCESSENTRY32W};

use util::from_wchar_buf;
use error::ErrorCode;
use Result;

use super::ProcessId;

/// See [`processes`](fn.processes.html).
#[derive(Debug)]
pub struct EnumProcesses(HANDLE, bool);
impl EnumProcesses {
	fn create() -> Result<EnumProcesses> {
		let handle = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };
		if handle == INVALID_HANDLE_VALUE {
			Err(ErrorCode::last())
		}
		else {
			Ok(EnumProcesses(handle, false))
		}
	}
}
impl Iterator for EnumProcesses {
	type Item = ProcessEntry;
	fn next(&mut self) -> Option<ProcessEntry> {
		unsafe {
			let mut entry: ProcessEntry = mem::uninitialized();
			entry.0.dwSize = mem::size_of::<PROCESSENTRY32W>() as DWORD;
			let result = if self.1 {
				Process32NextW(self.0, &mut entry.0)
			}
			else {
				self.1 = true;
				Process32FirstW(self.0, &mut entry.0)
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
impl Drop for EnumProcesses {
	fn drop(&mut self) {
		unsafe { CloseHandle(self.0); }
	}
}

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
	pub fn exe_file(&self) -> OsString {
		OsString::from_wide(from_wchar_buf(&self.0.szExeFile[..]))
	}
}
impl fmt::Debug for ProcessEntry {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("ProcessEntry")
			.field("dwSize", &self.0.dwSize)
			//.field("cntUsage", &self.0.cntUsage)
			.field("th32ProcessID", &self.0.th32ProcessID)
			//.field("th32DefaultHeapID", &self.0.th32DefaultHeapID)
			//.field("th32ModuleID", &self.0.th32ModuleID)
			.field("cntThreads", &self.0.cntThreads)
			.field("th32ParentProcessID", &self.0.th32ParentProcessID)
			.field("pcPriClassBase", &self.0.pcPriClassBase)
			//.field("dwFlags", &self.0.dwFlags)
			.field("szExeFile", &String::from_utf16(from_wchar_buf(&self.0.szExeFile[..])))
			.finish()
	}
}

/// Iterate over the running processes.
pub fn processes() -> Result<EnumProcesses> {
	EnumProcesses::create()
}
