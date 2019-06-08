use std::{mem, ptr};
use crate::process::ProcessEnvironmentBlock;

#[cfg(target_pointer_width = "32")]
macro_rules! ptr {
	(x86: $offset:literal, x64: $_:literal) => {
		unsafe {
			/*
				mov eax, fs:offset
				ret
			*/
			#[link_section = ".text"]
			static SHELLCODE: [u8; 7] = [0x64, 0xA1, ($offset & 0xff) as u8, (($offset >> 8) & 0xff) as u8, 0x00, 0x00, 0xC3];
			let f: unsafe fn() -> usize = mem::transmute(&SHELLCODE);
			f()
		}
	};
}

#[cfg(target_pointer_width = "64")]
macro_rules! ptr {
	(x86: $_:literal, x64: $offset:literal) => {
		unsafe {
			#[link_section = ".text"]
			static SHELLCODE: [u8; 10] = [0x65, 0x48, 0x8B, 0x04, 0x25, ($offset & 0xff) as u8, (($offset >> 8) & 0xff) as u8, 0x00, 0x00, 0xC3];
			let f: unsafe fn() -> usize = mem::transmute(&SHELLCODE);
			f()
		}
	};
}

#[repr(C)]
struct TEB {

}

#[derive(Copy, Clone)]
pub struct ThreadInformationBlock(*mut TEB);

impl ThreadInformationBlock {
	pub fn new() -> ThreadInformationBlock { ThreadInformationBlock(ptr::null_mut()) }

	pub fn process_environment_block(self) -> ProcessEnvironmentBlock {
		ProcessEnvironmentBlock::current()
	}
}
