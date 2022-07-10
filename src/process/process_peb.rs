use std::{fmt, ptr};
use crate::module::ModuleLoaderData;

#[repr(C)]
struct PEB {}

/// Process Environment Block.
#[derive(Copy, Clone)]
pub struct ProcessEnvironmentBlock(*mut PEB);
impl Default for ProcessEnvironmentBlock {
	#[inline]
	fn default() -> ProcessEnvironmentBlock {
		ProcessEnvironmentBlock::current()
	}
}
impl ProcessEnvironmentBlock {
	/// Gets the current Process Environment Block.
	#[inline]
	pub fn current() -> ProcessEnvironmentBlock {
		let peb;

		#[cfg(not(feature = "nightly"))]
		unsafe { let f: unsafe fn() -> *mut PEB = std::mem::transmute(&SHELLCODE); peb = f(); }

		#[cfg(target_pointer_width = "32")]
		#[cfg(feature = "nightly")]
		unsafe { llvm_asm!("mov $0, dword ptr fs:0x30" : "=r"(peb) : : : "intel"); }

		#[cfg(target_pointer_width = "64")]
		#[cfg(feature = "nightly")]
		unsafe { llvm_asm!("mov $0, qword ptr gs:0x60" : "=r"(peb) : : : "intel"); }

		ProcessEnvironmentBlock(peb)
	}
	/// Returns the value of `IsDebuggerPresent()`.
	#[inline]
	pub fn being_debugged(self) -> bool {
		unsafe { self.read::<u8>(0x02, 0x02) & 1 != 0 }
	}
	/// Returns the value of `GetModuleHandle(NULL)`.
	#[inline]
	pub fn image_base_address(self) -> *mut u8 {
		unsafe { self.read(0x08, 0x10) }
	}
	/// Gets information about the loaded modules.
	#[inline]
	pub fn loader_data(self) -> ModuleLoaderData {
		unsafe { ModuleLoaderData::new(self.read(0x0C, 0x18)) }
	}
	/// Returns the value of `GetProcessHeap()`.
	#[inline]
	pub fn process_heap(self) -> *mut u8 {
		unsafe { self.read(0x18, 0x30) }
	}
	/// Points to an array of function pointers to support KiUserCallbackDispatcher, invoked by KeUserModeCallback.
	#[inline]
	pub fn kernel_callback_table(self) -> *mut usize {
		unsafe { self.read(0x2C, 0x58) }
	}
	/// Pointer to the API Set Schema redirections.
	#[inline]
	pub fn api_set_schema(self) -> *const u8 {
		unsafe { self.read(0x38, 0x68) }
	}
	#[inline(always)]
	unsafe fn read<T>(self, _x86_offset: isize, _x64_offset: isize) -> T {
		#[cfg(target_pointer_width = "32")]
		return ptr::read((self.0 as *mut u8).offset(_x86_offset) as *mut T);
		#[cfg(target_pointer_width = "64")]
		return ptr::read((self.0 as *mut u8).offset(_x64_offset) as *mut T);
	}
}

impl fmt::Debug for ProcessEnvironmentBlock {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("ProcessEnvironmentBlock")
			.field("being_debugged", &self.being_debugged())
			.field("image_base_address", &self.image_base_address())
			.field("process_heap", &self.process_heap())
			.field("kernel_callback_table", &self.kernel_callback_table())
			.field("api_set_schema", &self.api_set_schema())
			.finish()
	}
}

//----------------------------------------------------------------

/*
	mov eax, fs:0x30
	ret
*/
#[cfg(all(not(feature = "nightly"), target_pointer_width = "32"))]
#[link_section = ".text"]
static SHELLCODE: [u8; 7] = [0x64, 0xA1, 0x30, 0x00, 0x00, 0x00, 0xC3];

/*
	mov rax, gs:0x60
	ret
*/
#[cfg(all(not(feature = "nightly"), target_pointer_width = "64"))]
#[allow(unused)]
#[link_section = ".text"]
static SHELLCODE: [u8; 10] = [0x65, 0x48, 0x8B, 0x04, 0x25, 0x60, 0x00, 0x00, 0x00, 0xC3];

//----------------------------------------------------------------

#[test]
fn peb() {
	let peb = ProcessEnvironmentBlock::current();
	println!("{:#?}", peb);
}
