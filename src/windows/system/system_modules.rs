use std::{cmp, fmt, mem, ops, slice};
use std::path::Path;

use ntapi::ntexapi::*;
use ntapi::ntldr::*;
use winapi::shared::ntdef::{PVOID, ULONG};

use crate::{AsInner, util};

//----------------------------------------------------------------

/// List of loaded system modules.
#[derive(Clone)]
pub struct SystemModules(Vec<u8>);
impl SystemModules {
	/// Constructor.
	#[inline(never)]
	pub fn query() -> SystemModules {
		let mut data = Vec::new();
		let mut return_length = 0;
		unsafe {
			loop {
				let ntstatus = NtQuerySystemInformation(
					SystemModuleInformation,
					data.as_mut_ptr() as PVOID,
					data.capacity() as ULONG,
					&mut return_length,
				);
				if ntstatus >= 0 {
					data.set_len(return_length as usize);
					break;
				}
				if data.capacity() < return_length as usize {
					let additional = return_length as usize - data.capacity();
					data.reserve_exact(additional);
				}
			}
		}
		SystemModules(data)
	}
}
impl AsInner<[RTL_PROCESS_MODULE_INFORMATION]> for SystemModules {
	fn as_inner(&self) -> &[RTL_PROCESS_MODULE_INFORMATION] {
		unsafe {
			let process_modules: *const RTL_PROCESS_MODULES = self.0.as_ptr() as *const _;
			let len = (*process_modules).NumberOfModules as usize;
			let p = (*process_modules).Modules.as_ptr();
			slice::from_raw_parts(p, len)
		}
	}
}
impl ops::Deref for SystemModules {
	type Target = [SystemModule];
	fn deref(&self) -> &[SystemModule] {
		unsafe { mem::transmute(self.as_inner()) }
	}
}
impl AsRef<[SystemModule]> for SystemModules {
	fn as_ref(&self) -> &[SystemModule] {
		unsafe { mem::transmute(self.as_inner()) }
	}
}
impl<'a> IntoIterator for &'a SystemModules {
	type Item = &'a SystemModule;
	type IntoIter = slice::Iter<'a, SystemModule>;
	fn into_iter(self) -> slice::Iter<'a, SystemModule> {
		self.iter()
	}
}
impl fmt::Debug for SystemModules {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_list()
			.entries(self.as_ref())
			.finish()
	}
}

//----------------------------------------------------------------

/// Loaded system module.
#[repr(C)]
pub struct SystemModule(RTL_PROCESS_MODULE_INFORMATION);
impl_inner!(SystemModule: RTL_PROCESS_MODULE_INFORMATION);
impl SystemModule {
	pub fn image_base(&self) -> usize {
		self.0.ImageBase as usize
	}
	pub fn image_size(&self) -> usize {
		self.0.ImageSize as usize
	}
	pub fn flags(&self) -> u32 {
		self.0.Flags
	}
	pub fn full_path_name(&self) -> &Path {
		unsafe {
			mem::transmute(util::from_char_buf(&self.0.FullPathName))
		}
	}
	pub fn file_name(&self) -> &Path {
		let offset = cmp::min(self.0.OffsetToFileName as usize, self.0.FullPathName.len());
		unsafe {
			mem::transmute(util::from_char_buf(&self.0.FullPathName[offset..]))
		}
	}
}
impl fmt::Debug for SystemModule {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("SystemModule")
			.field("image_base", &format_args!("{:#x}", self.image_base()))
			.field("image_size", &format_args!("{:#x}", self.image_size()))
			.field("flags", &self.flags())
			.field("file_name", &self.file_name())
			.field("full_path_name", &self.full_path_name())
			.finish()
	}
}

//----------------------------------------------------------------

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn units() {
		let modules = SystemModules::query();
		println!("{:#?}", modules);
	}
}
