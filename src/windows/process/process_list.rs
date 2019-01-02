use std::{cmp, fmt, mem, slice};
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

use ntdll::*;

use crate::FromInner;
use crate::thread::ThreadId;
use crate::process::ProcessId;
use crate::ptr::RawPtr;

//----------------------------------------------------------------

#[derive(Clone)]
pub struct ProcessList(Vec<u8>);
impl ProcessList {
	pub fn query() -> ProcessList {
		let mut data = Vec::new();
		let mut return_length = 0;
		unsafe {
			loop {
				let ntstatus = NtQuerySystemInformation(
					SystemProcessInformation,
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
		ProcessList(data)
	}
	pub fn iter<'a>(&'a self) -> ProcessListIter<'a> {
		ProcessListIter(&self.0)
	}
}
impl fmt::Debug for ProcessList {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_list().entries(self.iter()).finish()
	}
}

//----------------------------------------------------------------

pub struct ProcessListIter<'a>(&'a [u8]);
impl<'a> Iterator for ProcessListIter<'a> {
	type Item = &'a ProcessInformation;
	fn next(&mut self) -> Option<&'a ProcessInformation> {
		if self.0.len() == 0 {
			return None;
		}
		unsafe {
			let p = self.0.as_ptr() as *const SYSTEM_PROCESS_INFORMATION;
			let mut size_of = cmp::min((*p).NextEntryOffset as usize, self.0.len());
			if size_of == 0 {
				size_of = self.0.len();
			}
			self.0 = self.0.get_unchecked(size_of..);
			let pi = mem::transmute(slice::from_raw_parts(p, (*p).NumberOfThreads as usize));
			Some(pi)
		}
	}
}

//----------------------------------------------------------------

#[repr(C)]
pub struct ProcessInformation {
	pi: SYSTEM_PROCESS_INFORMATION,
	ti: [SYSTEM_THREAD_INFORMATION],
}
impl ProcessInformation {
	pub fn image_name_wide(&self) -> &[u16] {
		unsafe {
			slice::from_raw_parts(self.pi.ImageName.Buffer, (self.pi.ImageName.Length as u32 >> 1) as usize)
		}
	}
	pub fn image_name(&self) -> OsString {
		OsString::from_wide(self.image_name_wide())
	}
	pub fn process_id(&self) -> ProcessId {
		unsafe {
			ProcessId::from_inner(self.pi.UniqueProcessId as usize as u32)
		}
	}
	pub fn threads(&self) -> &[ThreadInformation] {
		unsafe {
			mem::transmute(&self.ti)
		}
	}
}
impl fmt::Debug for ProcessInformation {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("ProcessInformation")
			.field("process_id", &self.process_id())
			.field("image_name", &self.image_name())
			.field("threads", &self.threads())
			.finish()
	}
}

//----------------------------------------------------------------

#[repr(C)]
pub struct ThreadInformation(SYSTEM_THREAD_INFORMATION);
impl ThreadInformation {
	pub fn start_address(&self) -> RawPtr {
		RawPtr::from(self.0.StartAddress as usize)
	}
	pub fn process_id(&self) -> ProcessId {
		unsafe {
			ProcessId::from_inner(self.0.ClientId.UniqueProcess as usize as u32)
		}
	}
	pub fn thread_id(&self) -> ThreadId {
		unsafe {
			ThreadId::from_inner(self.0.ClientId.UniqueThread as usize as u32)
		}
	}
	pub fn thread_state(&self) -> u32 {
		self.0.ThreadState
	}
	pub fn wait_reason(&self) -> u32 {
		self.0.WaitReason
	}
}
impl fmt::Debug for ThreadInformation {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("ThreadInformation")
			.field("thread_id", &self.thread_id())
			.field("process_id", &self.process_id())
			.field("start_address", &self.start_address())
			.field("thread_state", &self.thread_state())
			.field("wait_reason", &self.wait_reason())
			.finish()
	}
}

//----------------------------------------------------------------

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn units() {
		let processes = ProcessList::query();
		println!("{:#?}", processes);
	}
}
