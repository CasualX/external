/*!
*/

use std::{mem, fmt};

use kernel32::{CloseHandle, GetCurrentThread, OpenThread, GetThreadId, GetProcessIdOfThread, GetExitCodeThread, WaitForSingleObject};
use winapi::{HANDLE, DWORD, FALSE, TRUE, WAIT_FAILED};

use process::ProcessId;
use error::ErrorCode;
use {Result, IntoInner, FromInner};

//----------------------------------------------------------------

/// Wraps a thread identifier.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ThreadId(DWORD);
impl_inner!(ThreadId: DWORD);

//----------------------------------------------------------------

use ::winapi::{DELETE, READ_CONTROL, SYNCHRONIZE, WRITE_DAC, WRITE_OWNER};
//use ::winapi::{THREAD_ALL_ACCESS, THREAD_DIRECT_IMPERSONATION, THREAD_GET_CONTEXT, THREAD_IMPERSONATE, THREAD_QUERY_INFORMATION, THREAD_QUERY_LIMITED_INFORMATION,
//	THREAD_SET_CONTEXT, THREAD_SET_INFORMATION, THREAD_SET_LIMITED_INFORMATION, THREAD_SET_THREAD_TOKEN, THREAD_SUSPEND_RESUME, THREAD_TERMINATE};

/// Create thread access rights using the builder pattern.
///
/// See [Thread Security and Access Rights](https://msdn.microsoft.com/en-us/library/windows/desktop/ms686769.aspx) for more information.
pub struct ThreadRights(DWORD);
impl_inner!(ThreadRights: DWORD);
impl ThreadRights {
	pub fn new() -> ThreadRights {
		ThreadRights(0)
	}
	pub fn all_access() -> ThreadRights {
		ThreadRights(0x1F0FFB)
	}

	pub fn delete(self) -> ThreadRights {
		ThreadRights(self.0 | DELETE)
	}
	pub fn read_control(self) -> ThreadRights {
		ThreadRights(self.0 | READ_CONTROL)
	}
	pub fn synchronize(self) -> ThreadRights {
		ThreadRights(self.0 | SYNCHRONIZE)
	}
	pub fn write_dac(self) -> ThreadRights {
		ThreadRights(self.0 | WRITE_DAC)
	}
	pub fn write_owner(self) -> ThreadRights {
		ThreadRights(self.0 | WRITE_OWNER)
	}

	pub fn direct_impersonation(self) -> ThreadRights {
		ThreadRights(self.0 | 0x0200)
	}
	pub fn get_context(self) -> ThreadRights {
		ThreadRights(self.0 | 0x0008)
	}
	pub fn impersonate(self) -> ThreadRights {
		ThreadRights(self.0 | 0x0100)
	}
	pub fn query_information(self) -> ThreadRights {
		ThreadRights(self.0 | 0x0040)
	}
	pub fn query_limited_information(self) -> ThreadRights {
		ThreadRights(self.0 | 0x0800)
	}
	pub fn set_context(self) -> ThreadRights {
		ThreadRights(self.0 | 0x0010)
	}
	pub fn set_information(self) -> ThreadRights {
		ThreadRights(self.0 | 0x0020)
	}
	pub fn set_limited_information(self) -> ThreadRights {
		ThreadRights(self.0 | 0x0400)
	}
	pub fn set_thread_token(self) -> ThreadRights {
		ThreadRights(self.0 | 0x0080)
	}
	pub fn suspend_resume(self) -> ThreadRights {
		ThreadRights(self.0 | 0x0002)
	}
	pub fn terminate(self) -> ThreadRights {
		ThreadRights(self.0 | 0x0001)
	}
}

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

//----------------------------------------------------------------

use ::kernel32::{CreateToolhelp32Snapshot, Thread32First, Thread32Next};
use ::winapi::{LONG, THREADENTRY32, INVALID_HANDLE_VALUE, TH32CS_SNAPTHREAD};

/// See [`threads`](fn.threads.html).
#[derive(Debug)]
pub struct EnumThreads(HANDLE, bool);
impl EnumThreads {
	fn create() -> Result<EnumThreads> {
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

/// Iterate over all running threads.
pub fn threads() -> Result<EnumThreads> {
	EnumThreads::create()
}
