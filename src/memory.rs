use std::{fmt, ops, ptr, mem};
use dataview::Pod;
use crate::winapi::*;
use crate::error::ErrorCode;
use crate::Result;

/// Memory protection.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Protect(u32);
impl_inner!(Protect: safe u32);
impl Protect {
	pub const EXECUTE: Protect = Protect(PAGE_EXECUTE);
	pub const EXECUTE_READ: Protect = Protect(PAGE_EXECUTE_READ);
	pub const EXECUTE_READWRITE: Protect = Protect(PAGE_EXECUTE_READWRITE);
	pub const NOACCESS: Protect = Protect(PAGE_NOACCESS);
	pub const READONLY: Protect = Protect(PAGE_READONLY);
	pub const READWRITE: Protect = Protect(PAGE_READWRITE);
}
impl Protect {
	#[inline]
	pub const fn is_executable(self) -> bool {
		self.0 & (PAGE_EXECUTE | PAGE_EXECUTE_READ | PAGE_EXECUTE_READWRITE | PAGE_EXECUTE_WRITECOPY) != 0
	}
	#[inline]
	pub const fn is_readable(self) -> bool {
		self.0 & (PAGE_EXECUTE_READ | PAGE_EXECUTE_READWRITE | PAGE_EXECUTE_WRITECOPY | PAGE_READONLY | PAGE_READWRITE | PAGE_WRITECOPY) != 0
	}
	#[inline]
	pub const fn is_writable(self) -> bool {
		self.0 & (PAGE_EXECUTE_READWRITE | PAGE_EXECUTE_WRITECOPY | PAGE_READWRITE | PAGE_WRITECOPY) != 0
	}
	#[inline]
	pub const fn has_guard(self) -> bool {
		self.0 & (PAGE_GUARD) != 0
	}
	#[inline]
	pub fn set_guard(self, value: bool) -> Protect {
		if value {
			Protect(self.0 | PAGE_GUARD)
		}
		else {
			Protect(self.0 & !PAGE_GUARD)
		}
	}
}
impl fmt::Debug for Protect {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("Protect")
			.field("flags", &format_args!("{:#x}", self.0))
			.field("is_executable", &self.is_executable())
			.field("is_readable", &self.is_readable())
			.field("is_writable", &self.is_writable())
			.field("has_guard", &self.has_guard())
			.finish()
	}
}

//----------------------------------------------------------------

/// Free type for virtual memory.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct FreeType(u32);
impl_inner!(FreeType: safe u32);
impl FreeType {
	pub const DECOMMIT: FreeType = FreeType(MEM_DECOMMIT);
	pub const RELEASE: FreeType = FreeType(MEM_RELEASE);
}

/// Allocation type for virtual memory.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct AllocType(u32);
impl_inner!(AllocType: safe u32);
impl AllocType {
	pub const COMMIT: AllocType = AllocType(MEM_COMMIT);
	pub const RESERVE: AllocType = AllocType(MEM_RESERVE);
	pub const RESET: AllocType = AllocType(MEM_RESET);
	pub const RESET_UNDO: AllocType = AllocType(MEM_RESET_UNDO);
}

/// Memory types from MemoryInformation.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct MemoryType(u32);
impl_inner!(MemoryType: safe u32);
impl MemoryType {
	pub const IMAGE: MemoryType = MemoryType(0x1000000);
	pub const MAPPED: MemoryType = MemoryType(0x40000);
	pub const PRIVATE: MemoryType = MemoryType(0x20000);
}

pub struct MemoryInformation(MEMORY_BASIC_INFORMATION);
impl_inner!(MemoryInformation: MEMORY_BASIC_INFORMATION);
impl ops::Deref for MemoryInformation {
	type Target = MEMORY_BASIC_INFORMATION;
	fn deref(&self) -> &MEMORY_BASIC_INFORMATION {
		&self.0
	}
}

#[derive(Copy, Clone, Default)]
pub struct WorkingSetExBlock(usize);
impl_inner!(WorkingSetExBlock: usize);
impl From<PSAPI_WORKING_SET_EX_BLOCK> for WorkingSetExBlock {
	fn from(ws_ex_block: PSAPI_WORKING_SET_EX_BLOCK) -> WorkingSetExBlock {
		WorkingSetExBlock(ws_ex_block.Flags)
	}
}
impl WorkingSetExBlock {
	pub const fn valid(&self) -> bool {
		self.0 & 1 != 0
	}
	pub const fn share_count(&self) -> u32 {
		((self.0 >> 1) & 0x7) as u32
	}
	pub const fn win32_protection(&self) -> Protect {
		Protect((self.0 >> 4) as u32)
	}
	pub const fn shared(&self) -> bool {
		self.0 & (1 << 15) != 0
	}
	pub const fn node(&self) -> u32 {
		((self.0 >> 16) & 0x3f) as u32
	}
	pub const fn locked(&self) -> bool {
		self.0 & (1 << 22) != 0
	}
	pub const fn large_page(&self) -> bool {
		self.0 & (1 << 23) != 0
	}
	pub const fn bad(&self) -> bool {
		self.0 & (1 << 31) != 0
	}
}
impl fmt::Debug for WorkingSetExBlock {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("WorkingSetExBlock")
			.field("valid", &self.valid())
			.field("share_count", &self.share_count())
			.field("win32_protection", &self.win32_protection())
			.field("shared", &self.shared())
			.field("node", &self.node())
			.field("locked", &self.locked())
			.field("large_page", &self.large_page())
			.field("bad", &self.bad())
			.finish()
	}
}
