use std::{mem, fmt};
use std::ffi::{OsString};
use std::os::windows::ffi::{OsStringExt};

use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use winapi::um::tlhelp32::{CreateToolhelp32Snapshot, Module32FirstW, Module32NextW, MODULEENTRY32W, TH32CS_SNAPMODULE, TH32CS_SNAPMODULE32};
use winapi::shared::ntdef::{HANDLE};
use winapi::shared::minwindef::{DWORD, FALSE, HMODULE};

use process::ProcessId;
use error::ErrorCode;
use util::from_wchar_buf;
use ptr::RawPtr;
use {Result, IntoInner, FromInner};

/// Module enumeration.
///
/// Uses the Toolhelp32 snapshot API.
#[derive(Debug)]
pub struct EnumModules(HANDLE, bool);
impl EnumModules {
	/// Creates an iterator over the modules in a process.
	pub fn create(pid: ProcessId) -> Result<EnumModules> {
		let handle = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, pid.into_inner()) };
		if handle == INVALID_HANDLE_VALUE {
			Err(ErrorCode::last())
		}
		else {
			Ok(EnumModules(handle, false))
		}
	}
}
impl Iterator for EnumModules {
	type Item = ModuleEntry;
	fn next(&mut self) -> Option<ModuleEntry> {
		unsafe {
			let mut entry: ModuleEntry = mem::uninitialized();
			entry.0.dwSize = mem::size_of::<MODULEENTRY32W>() as DWORD;
			let result = if self.1 {
				Module32NextW(self.0, &mut entry.0)
			}
			else {
				self.1 = true;
				Module32FirstW(self.0, &mut entry.0)
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
impl Drop for EnumModules {
	fn drop(&mut self) {
		unsafe { CloseHandle(self.0); }
	}
}

/// Module entry.
///
/// See [MODULEENTRY32](https://msdn.microsoft.com/en-us/library/windows/desktop/ms684225.aspx) for more information.
pub struct ModuleEntry(MODULEENTRY32W);
impl ModuleEntry {
	/// The identifier of the process whose modules are to be examined.
	pub fn process_id(&self) -> ProcessId {
		unsafe { ProcessId::from_inner(self.0.th32ProcessID) }
	}
	/// The base address of the module in the context of the owning process.
	pub fn base(&self) -> RawPtr {
		RawPtr::from(self.0.modBaseAddr as usize)
	}
	/// The size of the module, in bytes.
	pub fn size(&self) -> usize {
		self.0.modBaseSize as usize
	}
	/// A handle to the module in the context of the owning process.
	pub fn handle(&self) -> HMODULE {
		self.0.hModule
	}
	/// The module name.
	pub fn name_wide(&self) -> &[u16] {
		from_wchar_buf(&self.0.szModule)
	}
	/// The module name.
	pub fn name(&self) -> OsString {
		OsString::from_wide(self.name_wide())
	}
	/// The module path.
	pub fn exe_path_wide(&self) -> &[u16] {
		from_wchar_buf(&self.0.szExePath)
	}
	/// The module path.
	pub fn exe_path(&self) -> OsString {
		OsString::from_wide(self.exe_path_wide())
	}
}
impl fmt::Debug for ModuleEntry {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("ModuleEntry")
			.field("dwSize", &self.0.dwSize)
			//.field("th32ModuleID", &self.0.th32ModuleID)
			.field("th32ProcessID", &self.0.th32ProcessID)
			//.field("GlblcntUsage", &self.0.GlblcntUsage)
			//.field("ProccntUsage", &self.0.ProccntUsage)
			.field("modBaseAddr", &self.0.modBaseAddr)
			.field("modBaseSize", &(self.0.modBaseSize as *const ()))
			.field("hModule", &self.0.hModule)
			.field("szModule", &self.name())
			.field("szExePath", &self.exe_path())
			.finish()
	}
}
