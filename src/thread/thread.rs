use std::mem;
use crate::winapi::*;
use crate::process::ProcessId;
use crate::thread::{ThreadId, ThreadRights};
use crate::error::ErrorCode;
use crate::{Result, IntoInner, FromInner};

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
	pub fn attach(tid: ThreadId, inherit: bool, access: ThreadRights) -> Result<Thread> {
		let handle = unsafe { OpenThread(access.into_inner(), if inherit { TRUE } else { FALSE }, tid.into_inner()) };
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
			let mut code = mem::MaybeUninit::<DWORD>::uninit();
			if GetExitCodeThread(self.0, code.as_mut_ptr()) != FALSE {
				let code = code.assume_init();
				Ok(if code == 259/*STILL_ACTIVE*/ { None } else { Some(code) })
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
    /// Suspends the thread by increasing its suspend count by one.
	pub fn suspend(&self) -> Result<DWORD> {
		unsafe {
			let result = SuspendThread(self.0);
			if result == u32::MAX {
				Err(ErrorCode::last())
			} else {
				Ok(result)
			}
		}
	}
	/// Decrements the threads suspend count.
	/// When the suspend count is decremented to zero, the execution of the thread is resumed.
	pub fn resume(&self) -> Result<DWORD> {
		unsafe {
			let result = ResumeThread(self.0);
			if result == u32::MAX {
				Err(ErrorCode::last())
			} else {
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
