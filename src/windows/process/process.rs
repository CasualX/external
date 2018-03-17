use std::{mem, ptr};
use std::ffi::OsString;
use std::os::windows::ffi::{OsStringExt};

use winapi::um::processthreadsapi::{OpenProcess, GetCurrentProcess, GetProcessId, GetExitCodeProcess, CreateRemoteThread};
use winapi::um::winbase::{QueryFullProcessImageNameW, WAIT_FAILED};
use winapi::um::synchapi::{WaitForSingleObject};
use winapi::um::handleapi::{DuplicateHandle, CloseHandle};
use winapi::um::winnt::{DUPLICATE_SAME_ACCESS};
use winapi::shared::ntdef::{HANDLE, WCHAR};
use winapi::shared::minwindef::{LPVOID, DWORD, TRUE, FALSE};

use process::{ProcessId, ProcessRights};
use module::{EnumModules, modules};
use thread::Thread;
use ptr::RawPtr;
use error::ErrorCode;
use {Result, IntoInner, FromInner};

/// Process handle.
#[derive(Debug)]
pub struct Process(HANDLE);
impl_inner!(Process: HANDLE);
impl Process {
	/// Get the current process.
	pub fn current() -> Process {
		Process(unsafe { GetCurrentProcess() })
	}
	/// Attach to a process by id and given rights.
	pub fn attach(pid: ProcessId, rights: ProcessRights) -> Result<Process> {
		// FIXME! What about handle inheritance?
		let handle = unsafe { OpenProcess(rights.into_inner(), TRUE, pid.into_inner()) };
		if handle.is_null() {
			Err(ErrorCode::last())
		}
		else {
			Ok(Process(handle))
		}
	}
	/// Get the id for this process.
	pub fn pid(&self) -> Result<ProcessId> {
		let pid = unsafe { GetProcessId(self.0) };
		if pid != 0 {
			Ok(ProcessId(pid))
		}
		else {
			Err(ErrorCode::last())
		}
	}
	/// Get the exit code for the process, `None` if the process is still running.
	pub fn exit_code(&self) -> Result<Option<DWORD>> {
		unsafe {
			let mut code: DWORD = mem::uninitialized();
			if GetExitCodeProcess(self.0, &mut code) != FALSE {
				Ok(if code == 259/*STILL_ACTIVE*/ { None } else { Some(code) })
			}
			else {
				Err(ErrorCode::last())
			}
		}
	}
	/// Wait for the process to finish.
	///
	/// See [WaitForSingleObject](https://msdn.microsoft.com/en-us/library/windows/desktop/ms687032.aspx) for more information.
	pub fn wait(&self, milis: DWORD) -> Result<DWORD> {
		unsafe {
			let result = WaitForSingleObject(self.0, milis);
			if result == WAIT_FAILED {
				Err(ErrorCode::last())
			}
			else {
				Ok(result)
			}
		}
	}
	pub fn create_thread(&self, start_address: RawPtr, parameter: RawPtr) -> Result<Thread> {
		unsafe {
			let start_address: usize = start_address.into();
			let parameter: usize = parameter.into();
			let handle = CreateRemoteThread(self.0, ptr::null_mut(), 0, mem::transmute(start_address), parameter as LPVOID, 0, ptr::null_mut());
			if handle.is_null() {
				Err(ErrorCode::last())
			}
			else {
				Ok(Thread::from_inner(handle))
			}
		}
	}
	/// Get the full name of the executable for this process.
	pub fn full_image_name(&self) -> Result<OsString> {
		unsafe {
			let mut buf: [WCHAR; 1000] = mem::uninitialized();
			let mut size = 1000;
			if QueryFullProcessImageNameW(self.0, 0, buf.as_mut_ptr(), &mut size) != FALSE {
				Ok(OsString::from_wide(&buf[..size as usize]))
			}
			else {
				Err(ErrorCode::last())
			}
		}
	}
	/// Iterate over the modules in this process.
	pub fn modules(&self) -> Result<EnumModules> {
		modules(try!(self.pid()))
	}
}
impl Clone for Process {
	fn clone(&self) -> Process {
		Process(unsafe {
			let current = GetCurrentProcess();
			let mut new: HANDLE = mem::uninitialized();
			// What about all these options? inherit handles?
			let result = DuplicateHandle(current, self.0, current, &mut new, 0, FALSE, DUPLICATE_SAME_ACCESS);
			// Can't report error, should this ever fail?
			assert!(result != FALSE, "duplicate handle error: {}", ErrorCode::last());
			new
		})
	}
}
impl Drop for Process {
	fn drop(&mut self) {
		unsafe { CloseHandle(self.0); }
	}
}
