/*!
*/

use std::{mem};

use winapi::um::handleapi::{CloseHandle};
use winapi::um::synchapi::{WaitForSingleObject};
use winapi::um::processthreadsapi::{GetCurrentThread, OpenThread, GetThreadId, GetProcessIdOfThread, GetExitCodeThread};
use winapi::um::winbase::{WAIT_FAILED};
use winapi::shared::ntdef::{HANDLE};
use winapi::shared::minwindef::{DWORD, FALSE, TRUE};

use process::ProcessId;
use thread::{ThreadId, ThreadRights};
use error::ErrorCode;
use {Result, IntoInner, FromInner};

//----------------------------------------------------------------

/// Thread handle.
#[derive(Debug)]
pub struct Thread(HANDLE);
impl_inner!(Thread: HANDLE);
impl Thread {
	/// Get the current thread.
	pub fn current() -> Thread {
		Thread(unsafe { GetCurrentThread() })
	}
	/// Attach to a thread by id and given rights.
	pub fn attach(tid: ThreadId, access: ThreadRights) -> Result<Thread> {
		// FIXME! What about handle inheritance?
		let handle = unsafe { OpenThread(access.into_inner(), TRUE, tid.into_inner()) };
		if handle.is_null() {
			Err(ErrorCode::last())
		}
		else {
			Ok(Thread(handle))
		}
	}
	/// Get the id for this thread.
	pub fn tid(&self) -> Result<ThreadId> {
		let tid = unsafe { GetThreadId(self.0) };
		if tid != 0 {
			Ok(ThreadId(tid))
		}
		else {
			Err(ErrorCode::last())
		}
	}
	/// Get the owning process' id.
	pub fn process_id(&self) -> Result<ProcessId> {
		let pid = unsafe { GetProcessIdOfThread(self.0) };
		if pid != 0 {
			Ok(unsafe { ProcessId::from_inner(pid) })
		}
		else {
			Err(ErrorCode::last())
		}
	}
	/// Get the exit code for the thread, `None` if the thread is still running.
	pub fn exit_code(&self) -> Result<Option<DWORD>> {
		unsafe {
			let mut code: DWORD = mem::uninitialized();
			if GetExitCodeThread(self.0, &mut code) != FALSE {
				if code == 259/*STILL_ACTIVE*/ {
					Ok(None)
				}
				else {
					Ok(Some(code))
				}
			}
			else {
				Err(ErrorCode::last())
			}
		}
	}
	/// Wait for the thread to finish.
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
}
impl Drop for Thread {
	fn drop(&mut self) {
		unsafe {
			CloseHandle(self.0);
		}
	}
}
