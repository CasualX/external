use std::{fmt, mem};
use crate::winapi::*;

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
impl fmt::Debug for LDR_DATA_ENTRY {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("LDR_DATA_ENTRY")
			.field("InLoadOrderModuleList.Flink", &self.InLoadOrderModuleList.Flink)
			.field("InLoadOrderModuleList.Blink", &self.InLoadOrderModuleList.Blink)
			.field("InMemoryOrderModuleList.Flink", &self.InMemoryOrderModuleList.Flink)
			.field("InMemoryOrderModuleList.Blink", &self.InMemoryOrderModuleList.Blink)
			.field("InInitializationOrderModuleList.Flink", &self.InInitializationOrderModuleList.Flink)
			.field("InInitializationOrderModuleList.Blink", &self.InInitializationOrderModuleList.Blink)
			.field("BaseAddress", &self.BaseAddress)
			.field("EntryPoint", &self.EntryPoint)
			.field("SizeOfImage", &self.SizeOfImage)
			.field("Flags", &self.Flags)
			.field("LoadCount", &self.LoadCount)
			.field("TlsIndex", &self.TlsIndex)
			.finish()
	}
}
#[allow(non_snake_case)]
#[repr(C)]
struct PEB_LDR_DATA
{
	pub Length: ULONG,
	pub Initialized: UCHAR,
	pub SsHandle: PVOID,
	pub InLoadOrderModuleList: LIST_ENTRY,
	pub InMemoryOrderModuleList: LIST_ENTRY,
	pub InInitializationOrderModuleList: LIST_ENTRY,
	pub EntryInProgress: PVOID,
}
impl fmt::Debug for PEB_LDR_DATA {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("PEB_LDR_DATA")
			.field("Length", &self.Length)
			.field("Initialized", &self.Initialized)
			.field("SsHandle", &self.SsHandle)
			.field("InLoadOrderModuleList.Flink", &self.InLoadOrderModuleList.Flink)
			.field("InLoadOrderModuleList.Blink", &self.InLoadOrderModuleList.Blink)
			.field("InMemoryOrderModuleList.Flink", &self.InMemoryOrderModuleList.Flink)
			.field("InMemoryOrderModuleList.Blink", &self.InMemoryOrderModuleList.Blink)
			.field("InInitializationOrderModuleList.Flink", &self.InInitializationOrderModuleList.Flink)
			.field("InInitializationOrderModuleList.Blink", &self.InInitializationOrderModuleList.Blink)
			.field("EntryInProgress", &self.EntryInProgress)
			.finish()
	}
}

//----------------------------------------------------------------

pub struct MemoryOrderModuleIter {
	it: *mut LDR_DATA_ENTRY,
	end: *mut LDR_DATA_ENTRY,
}
impl MemoryOrderModuleIter {
	/// Creates a new iterator.
	///
	/// Does not obtain the loader lock, do not keep the iterator while loading or unloading libraries.
	pub unsafe fn new() -> MemoryOrderModuleIter {
		let peb_ldr_data = get_peb_ldr_data();
		// println!("{:?} {:#?}", peb_ldr_data, &mut *peb_ldr_data);
		MemoryOrderModuleIter {
			it: (*peb_ldr_data).InLoadOrderModuleList.Flink as *mut LDR_DATA_ENTRY,
			end: &mut (*peb_ldr_data).InLoadOrderModuleList as *mut _ as *mut LDR_DATA_ENTRY,
		}
	}
}
impl Iterator for MemoryOrderModuleIter {
	type Item = ModuleDataEntry;
	fn next(&mut self) -> Option<ModuleDataEntry> {
		if self.it == self.end {
			return None;
		}
		let data_entry = self.it;
		println!("{:#?}", unsafe { &*data_entry });
		self.it = unsafe { (*self.it).InLoadOrderModuleList.Flink as *mut LDR_DATA_ENTRY };
		Some(ModuleDataEntry { data_entry })
	}
}

/*
      mov eax, fs:[0x30]
      mov eax, [eax+0x0C]
      ret
*/
#[cfg(target_pointer_width = "32")]
#[link_section = ".text"]
static GET_PEB_LDR_DATA: [u8; 10] = [0x64, 0xA1, 0x30, 0x00, 0x00, 0x00, 0x8B, 0x40, 0x0C, 0xC3];

/*
    mov rax, gs:[0x30]
    mov rax, [rax+0x60]
    ret
*/
#[cfg(target_pointer_width = "64")]
#[link_section = ".text"]
static GET_PEB_LDR_DATA: [u8; 14] = [0x65, 0x48, 0x8B, 0x04, 0x25, 0x30, 0x00, 0x00, 0x00, 0x48, 0x8B, 0x40, 0x60, 0xC3];

fn get_peb_ldr_data() -> *mut PEB_LDR_DATA {
	unsafe {
		mem::transmute::<_, unsafe extern "cdecl" fn() -> *mut PEB_LDR_DATA>(&GET_PEB_LDR_DATA)()
	}
}

//----------------------------------------------------------------

pub struct ModuleDataEntry {
	data_entry: *mut LDR_DATA_ENTRY,
}
impl ModuleDataEntry {
	pub fn base_address(&self) -> usize {
		unsafe { (*self.data_entry).BaseAddress as usize }
	}
	pub fn entry_point(&self) -> usize {
		unsafe { (*self.data_entry).EntryPoint as usize }
	}
}
impl fmt::Debug for ModuleDataEntry {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("ModuleDataEntry")
			.field("base_address", &format_args!("{:#x}", self.base_address()))
			.field("entry_point", &format_args!("{:#x}", self.entry_point()))
			.finish()
	}
}

//----------------------------------------------------------------

/*
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn units() {
		for entry in unsafe { MemoryOrderModuleIter::new() } {
			println!("{:?}", entry);
		}
	}
}
*/
