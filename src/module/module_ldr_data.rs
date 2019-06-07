use std::{fmt, slice};
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use crate::winapi::*;

//----------------------------------------------------------------

#[allow(non_snake_case)]
#[repr(C)]
struct PEB_LDR_DATA {
	pub Length: ULONG,
	pub Initialized: UCHAR,
	pub SsHandle: PVOID,
	pub InLoadOrderModuleList: LIST_ENTRY,
	pub InMemoryOrderModuleList: LIST_ENTRY,
	pub InInitializationOrderModuleList: LIST_ENTRY,
	pub EntryInProgress: PVOID,
}

/// Module loader data.
#[derive(Copy, Clone)]
pub struct ModuleLoaderData {
	ptr: *mut PEB_LDR_DATA,
}
impl ModuleLoaderData {
	pub(crate) unsafe fn new(address: usize) -> ModuleLoaderData {
		let ptr = address as *mut PEB_LDR_DATA;
		ModuleLoaderData { ptr }
	}
	pub fn length(self) -> usize {
		unsafe { (*self.ptr).Length as usize }
	}
	pub fn initialized(self) -> bool {
		unsafe { (*self.ptr).Initialized != 0 }
	}
	pub fn load_order(self) -> InLoadOrderModuleList {
		unsafe {
			let it = (*self.ptr).InLoadOrderModuleList.Flink;
			let end = &mut (*self.ptr).InLoadOrderModuleList as *mut _;
			InLoadOrderModuleList { it, end }
		}
	}
	pub fn memory_order(self) -> InMemoryOrderModuleList {
		unsafe {
			let it = (*self.ptr).InMemoryOrderModuleList.Flink;
			let end = &mut (*self.ptr).InMemoryOrderModuleList as *mut _;
			InMemoryOrderModuleList { it, end }
		}
	}
	pub fn initialization_order(self) -> InInitializationOrderModuleList {
		unsafe {
			let it = (*self.ptr).InInitializationOrderModuleList.Flink;
			let end = &mut (*self.ptr).InInitializationOrderModuleList as *mut _;
			InInitializationOrderModuleList { it, end }
		}
	}
}

//----------------------------------------------------------------

/// Load order module list iterator.
#[derive(Clone)]
pub struct InLoadOrderModuleList {
	it: *mut LIST_ENTRY,
	end: *mut LIST_ENTRY,
}
impl Iterator for InLoadOrderModuleList {
	type Item = ModuleDataEntry;
	fn next(&mut self) -> Option<ModuleDataEntry> {
		if self.it == self.end {
			return None;
		}
		unsafe {
			let ptr = self.it.offset(0) as *mut LDR_DATA_ENTRY;
			self.it = (*self.it).Flink;
			Some(ModuleDataEntry { ptr })
		}
	}
}

/// Memory order module list iterator.
#[derive(Clone)]
pub struct InMemoryOrderModuleList {
	it: *mut LIST_ENTRY,
	end: *mut LIST_ENTRY,
}
impl Iterator for InMemoryOrderModuleList {
	type Item = ModuleDataEntry;
	fn next(&mut self) -> Option<ModuleDataEntry> {
		if self.it == self.end {
			return None;
		}
		unsafe {
			let ptr = self.it.offset(-1) as *mut LDR_DATA_ENTRY;
			self.it = (*self.it).Flink;
			Some(ModuleDataEntry { ptr })
		}
	}
}

/// Initialization order module list iterator.
#[derive(Clone)]
pub struct InInitializationOrderModuleList {
	it: *mut LIST_ENTRY,
	end: *mut LIST_ENTRY,
}
impl Iterator for InInitializationOrderModuleList {
	type Item = ModuleDataEntry;
	fn next(&mut self) -> Option<ModuleDataEntry> {
		if self.it == self.end {
			return None;
		}
		unsafe {
			let ptr = self.it.offset(-2) as *mut LDR_DATA_ENTRY;
			self.it = (*self.it).Flink;
			Some(ModuleDataEntry { ptr })
		}
	}
}

//----------------------------------------------------------------

#[allow(non_snake_case)]
#[repr(C)]
struct LDR_DATA_ENTRY {
	pub InLoadOrderModuleList: LIST_ENTRY,
	pub InMemoryOrderModuleList: LIST_ENTRY,
	pub InInitializationOrderModuleList: LIST_ENTRY,
	pub BaseAddress: PVOID,
	pub EntryPoint: PVOID,
	pub SizeOfImage: ULONG,
	pub FullDllName: UNICODE_STRING,
	pub BaseDllName: UNICODE_STRING,
	pub Flags: ULONG,
	pub LoadCount: SHORT,
	pub TlsIndex: SHORT,
	pub HashTableEntry: LIST_ENTRY,
	pub TimeDateStamp: ULONG,
}

#[derive(Copy, Clone)]
pub struct ModuleDataEntry {
	ptr: *mut LDR_DATA_ENTRY,
}
impl ModuleDataEntry {
	pub fn base_address(self) -> *mut u8 {
		unsafe { (*self.ptr).BaseAddress as *mut u8 }
	}
	pub fn entry_point(self) -> *mut u8 {
		unsafe { (*self.ptr).EntryPoint as *mut u8 }
	}
	pub fn size_of_image(self) -> usize {
		unsafe { (*self.ptr).SizeOfImage as usize }
	}
	pub fn full_dll_name_wide(self) -> *mut [u16] {
		unsafe {
			let us = (*self.ptr).FullDllName;
			slice::from_raw_parts_mut(us.Buffer, (us.Length / 2) as usize)
		}
	}
	pub fn full_dll_name(self) -> OsString {
		OsString::from_wide(unsafe { &*self.full_dll_name_wide() })
	}
	pub fn base_dll_name_wide(self) -> *mut [u16] {
		unsafe {
			let us = (*self.ptr).BaseDllName;
			slice::from_raw_parts_mut(us.Buffer, (us.Length / 2) as usize)
		}
	}
	pub fn base_dll_name(self) -> OsString {
		OsString::from_wide(unsafe { &*self.base_dll_name_wide() })
	}
	pub fn flags(self) -> u32 {
		unsafe { (*self.ptr).Flags }
	}
	pub fn load_count(self) -> i16 {
		unsafe { (*self.ptr).LoadCount }
	}
	pub fn tls_index(self) -> u32 {
		unsafe { (*self.ptr).TlsIndex as u16 as u32 }
	}
	pub fn time_date_stamp(self) -> u32 {
		unsafe { (*self.ptr).TimeDateStamp }
	}
}
impl fmt::Debug for ModuleDataEntry {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("ModuleDataEntry")
			.field("base_address", &self.base_address())
			.field("entry_point", &self.entry_point())
			.field("size_of_image", &format_args!("{:#x}", self.size_of_image()))
			.field("full_dll_name", &self.full_dll_name())
			.field("base_dll_name", &self.base_dll_name())
			.field("flags", &format_args!("{:#x}", self.flags()))
			.field("load_count", &self.load_count())
			.field("tls_index", &self.tls_index())
			.field("time_date_stamp", &self.time_date_stamp())
			.finish()
	}
}

//----------------------------------------------------------------

#[test]
fn memory_order() {
	let peb = crate::process::ProcessEnvironmentBlock::current();
	println!("InMemoryOrderModuleList\n{:#?}", peb);
	for entry in peb.loader_data().memory_order() {
		println!("{:#?}", entry);
	}
}

#[test]
fn load_order() {
	let peb = crate::process::ProcessEnvironmentBlock::current();
	println!("InLoadOrderModuleList\n{:#?}", peb);
	for entry in peb.loader_data().load_order() {
		println!("{:#?}", entry);
	}
}

#[test]
fn initialization_order() {
	let peb = crate::process::ProcessEnvironmentBlock::current();
	println!("InInitializationOrderModuleList\n{:#?}", peb);
	for entry in peb.loader_data().initialization_order() {
		println!("{:#?}", entry);
	}
}
