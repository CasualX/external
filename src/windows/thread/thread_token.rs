/*!
!*/

use std::{mem, ptr};

use advapi32::{AdjustTokenPrivileges};
use kernel32::{CloseHandle};
use winapi::{DWORD, HANDLE, PHANDLE, BOOL, LPCWSTR, PLUID, TOKEN_PRIVILEGES};
use winapi::{FALSE, SE_PRIVILEGE_ENABLED};
use winapi::{TOKEN_ALL_ACCESS, TOKEN_ASSIGN_PRIMARY, TOKEN_DUPLICATE, TOKEN_IMPERSONATE, TOKEN_QUERY,
	TOKEN_QUERY_SOURCE, TOKEN_ADJUST_PRIVILEGES, TOKEN_ADJUST_GROUPS, TOKEN_ADJUST_DEFAULT, TOKEN_ADJUST_SESSIONID};

use thread::Thread;
use error::{ErrorCode, ERROR_SUCCESS};
use {Result, AsInner, IntoInner};

//----------------------------------------------------------------

extern "system" {
	fn OpenThreadToken(ThreadHandle: HANDLE, DesiredAccess: DWORD, OpenAsSelf: BOOL, TokenHandle: PHANDLE) -> BOOL;
	fn LookupPrivilegeValueW(lpSystemName: LPCWSTR, lpName: LPCWSTR, lpLuid: PLUID) -> BOOL;
	fn ImpersonateSelf(ImpersonationLevel: u32) -> BOOL;
}

//----------------------------------------------------------------

/// Create token access rights using the builder pattern.
///
/// See [Access Rights for Access-Token Objects](https://msdn.microsoft.com/en-us/library/windows/desktop/aa374905.aspx) for more information.
#[derive(Copy, Clone, Debug)]
pub struct ThreadTokenRights(DWORD);
impl_inner!(ThreadTokenRights: DWORD);
impl ThreadTokenRights {
	pub fn new() -> ThreadTokenRights {
		ThreadTokenRights(0)
	}
	pub fn all_access() -> ThreadTokenRights {
		ThreadTokenRights(TOKEN_ALL_ACCESS)
	}

	pub fn assign_primary(self) -> ThreadTokenRights {
		ThreadTokenRights(self.0 | TOKEN_ASSIGN_PRIMARY)
	}
	pub fn duplicate(self) -> ThreadTokenRights {
		ThreadTokenRights(self.0 | TOKEN_DUPLICATE)
	}
	pub fn impersonate(self) -> ThreadTokenRights {
		ThreadTokenRights(self.0 | TOKEN_IMPERSONATE)
	}
	pub fn query(self) -> ThreadTokenRights {
		ThreadTokenRights(self.0 | TOKEN_QUERY)
	}
	pub fn query_source(self) -> ThreadTokenRights {
		ThreadTokenRights(self.0 | TOKEN_QUERY_SOURCE)
	}
	pub fn adjust_privileges(self) -> ThreadTokenRights {
		ThreadTokenRights(self.0 | TOKEN_ADJUST_PRIVILEGES)
	}
	pub fn adjust_groups(self) -> ThreadTokenRights {
		ThreadTokenRights(self.0 | TOKEN_ADJUST_GROUPS)
	}
	pub fn adjust_default(self) -> ThreadTokenRights {
		ThreadTokenRights(self.0 | TOKEN_ADJUST_DEFAULT)
	}
	pub fn adjust_sessionid(self) -> ThreadTokenRights {
		ThreadTokenRights(self.0 | TOKEN_ADJUST_SESSIONID)
	}
}

//----------------------------------------------------------------

#[derive(Copy, Clone, Debug)]
pub struct Privilege<'a>(&'a [u16]);

pub const SE_DEBUG_NAME: Privilege<'static> = Privilege(&wide_str!('S' 'e' 'D' 'e' 'b' 'u' 'g' 'P' 'r' 'i' 'v' 'i' 'l' 'e' 'g' 'e' 0));

#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum SecurityImpersonationLevel {
	SecurityAnonymous,
	SecurityIdentification,
	SecurityImpersonation,
	SecurityDelegation,
}
pub use self::SecurityImpersonationLevel::*;

//----------------------------------------------------------------

/// Thread token handle.
pub struct ThreadToken(HANDLE);
impl_inner!(ThreadToken: HANDLE);
impl ThreadToken {
	pub fn open(thread: &Thread, rights: ThreadTokenRights, open_as_self: bool) -> Result<ThreadToken> {
		unsafe {
			let mut handle = mem::uninitialized();
			if OpenThreadToken(*thread.as_inner(), rights.into_inner(), open_as_self as BOOL, &mut handle) != FALSE {
				Ok(ThreadToken(handle))
			}
			else {
				Err(ErrorCode::last())
			}
		}
	}
	pub fn set_privilege(&self, privilege: Privilege, enable: bool) -> Result<()> {
		// From MSDN: https://msdn.microsoft.com/en-us/library/windows/hardware/ff541528.aspx
		unsafe {
			let mut luid = mem::uninitialized();
			if LookupPrivilegeValueW(ptr::null(), privilege.0.as_ptr(), &mut luid) != FALSE {
				// FIXME! winapi-rs has incorrect TOKEN_PRIVILEGES...
				let mut tp = [1u32, luid.LowPart, luid.HighPart as u32, if enable { SE_PRIVILEGE_ENABLED } else { 0 }];
				let tp: &mut TOKEN_PRIVILEGES = mem::transmute(&mut tp);
				AdjustTokenPrivileges(self.0, FALSE, tp, 0, ptr::null_mut(), ptr::null_mut());
			}
			match ErrorCode::last() {
				ERROR_SUCCESS => Ok(()),
				err => Err(err),
			}
		}
	}
	pub fn impersonate_self(impersonation_level: SecurityImpersonationLevel) -> Result<()> {
		unsafe {
			if ImpersonateSelf(impersonation_level as u32) != FALSE {
				Ok(())
			}
			else {
				Err(ErrorCode::last())
			}
		}
	}
}
impl Drop for ThreadToken {
	fn drop(&mut self) {
		unsafe {
			CloseHandle(self.0);
		}
	}
}
