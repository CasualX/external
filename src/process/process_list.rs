use std::{cmp, fmt, mem, slice};
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use crate::winapi::*;
use crate::FromInner;
use crate::thread::ThreadId;
use crate::process::ProcessId;

//----------------------------------------------------------------

#[derive(Clone)]
pub struct ProcessList(Box<[u8]>);
impl ProcessList {
	#[inline(never)]
	pub fn query() -> ProcessList {
		let mut data = Vec::new().into_boxed_slice();
		let mut return_length = 0;
		unsafe {
			while NtQuerySystemInformation(
				SystemProcessInformation,
				data.as_mut_ptr() as PVOID,
				data.len() as ULONG,
				&mut return_length,
			) < 0 {
				data = vec![0; return_length as usize].into_boxed_slice();
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
		
			let p = self.0.as_ptr() as *const SYSTEM_PROCESS_INFORMATION;
			let mut size_of = unsafe {cmp::min((*p).NextEntryOffset as usize, self.0.len())};
			if size_of == 0 {
				size_of = self.0.len();
			}
			self.0 = unsafe {self.0.get_unchecked(size_of..)};
			let pi = unsafe {mem::transmute(slice::from_raw_parts(p, (*p).NumberOfThreads as usize))};
			Some(pi)
		
	}
}

//----------------------------------------------------------------

#[repr(C)]
pub struct ProcessInformation {
	pi: SYSTEM_PROCESS_INFORMATION,
	ti: [SYSTEM_THREAD_INFORMATION],
}
impl ProcessInformation {
	pub fn working_set_private_size(&self) -> u64 {
		unsafe { mem::transmute(self.pi.WorkingSetPrivateSize) }
	}
	pub fn hard_fault_count(&self) -> u32 {
		self.pi.HardFaultCount
	}
	pub fn cycle_time(&self) -> u64 {
		self.pi.CycleTime
	}
	pub fn create_time(&self) -> u64 {
		unsafe { mem::transmute(self.pi.CreateTime) }
	}
	pub fn user_time(&self) -> u64 {
		unsafe { mem::transmute(self.pi.UserTime) }
	}
	pub fn kernel_time(&self) -> u64 {
		unsafe { mem::transmute(self.pi.KernelTime) }
	}
	pub fn image_name_wide(&self) -> &[u16] {
		unsafe { slice::from_raw_parts(self.pi.ImageName.Buffer, (self.pi.ImageName.Length as u32 >> 1) as usize) }
	}
	pub fn image_name(&self) -> OsString {
		OsString::from_wide(self.image_name_wide())
	}
	pub fn process_id(&self) -> ProcessId {
		unsafe { ProcessId::from_inner(self.pi.UniqueProcessId as usize as u32) }
	}
	pub fn parent_process_id(&self) -> ProcessId {
		unsafe { ProcessId::from_inner(self.pi.InheritedFromUniqueProcessId as usize as u32) }
	}
	pub fn handle_count(&self) -> u32 {
		self.pi.HandleCount
	}
	pub fn session_id(&self) -> u32 {
		self.pi.SessionId
	}
	pub fn peak_virtual_size(&self) -> usize {
		self.pi.PeakVirtualSize
	}
	pub fn virtual_size(&self) -> usize {
		self.pi.VirtualSize
	}
	pub fn page_fault_count(&self) -> u32 {
		self.pi.PageFaultCount
	}
	pub fn peak_working_set_size(&self) -> usize {
		self.pi.PeakWorkingSetSize
	}
	pub fn working_set_size(&self) -> usize {
		self.pi.WorkingSetSize
	}
	pub fn pagefile_usage(&self) -> usize {
		self.pi.PagefileUsage
	}
	pub fn peak_pagefile_usage(&self) -> usize {
		self.pi.PeakPagefileUsage
	}
	pub fn private_page_count(&self) -> usize {
		self.pi.PrivatePageCount
	}
	pub fn threads(&self) -> &[ThreadInformation] {
		unsafe { mem::transmute(&self.ti) }
	}
}
impl fmt::Debug for ProcessInformation {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("ProcessInformation")
			.field("number_of_threads", &self.threads().len())
			.field("hard_fault_count", &self.hard_fault_count())
			.field("cycle_time", &self.cycle_time())
			.field("create_time", &self.create_time())
			.field("user_time", &self.user_time())
			.field("kernel_time", &self.kernel_time())
			.field("image_name", &self.image_name())
			.field("process_id", &self.process_id())
			.field("parent_process_id", &self.parent_process_id())
			.field("handle_count", &self.handle_count())
			.field("session_id", &self.session_id())
			.field("peak_virtual_size", &self.peak_virtual_size())
			.field("virtual_size", &self.virtual_size())
			.field("page_fault_count", &self.page_fault_count())
			.field("peak_working_set_size", &self.peak_working_set_size())
			.field("working_set_size", &self.working_set_size())
			.field("pagefile_usage", &self.pagefile_usage())
			.field("peak_pagefile_usage", &self.peak_pagefile_usage())
			.field("private_page_count", &self.private_page_count())
			.field("threads", &self.threads())
			.finish()
	}
}

//----------------------------------------------------------------

#[repr(C)]
pub struct ThreadInformation(SYSTEM_THREAD_INFORMATION);
impl ThreadInformation {
	pub fn kernel_time(&self) -> u64 {
		unsafe { mem::transmute(self.0.KernelTime) }
	}
	pub fn user_time(&self) -> u64 {
		unsafe { mem::transmute(self.0.UserTime) }
	}
	pub fn create_time(&self) -> u64 {
		unsafe { mem::transmute(self.0.CreateTime) }
	}
	pub fn wait_time(&self) -> u32 {
		self.0.WaitTime
	}
	pub fn start_address(&self) -> usize {
		self.0.StartAddress as usize
	}
	pub fn process_id(&self) -> ProcessId {
		unsafe { ProcessId::from_inner(self.0.ClientId.UniqueProcess as usize as u32) }
	}
	pub fn thread_id(&self) -> ThreadId {
		unsafe { ThreadId::from_inner(self.0.ClientId.UniqueThread as usize as u32) }
	}
	pub fn priority(&self) -> i32 {
		self.0.Priority
	}
	pub fn base_priority(&self) -> i32 {
		self.0.BasePriority
	}
	pub fn context_switches(&self) -> u32 {
		self.0.ContextSwitches
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
			.field("kernel_time", &self.kernel_time())
			.field("user_time", &self.user_time())
			.field("create_time", &self.create_time())
			.field("wait_time", &self.wait_time())
			.field("start_address", &format_args!("{:#x}", self.start_address()))
			.field("process_id", &self.process_id())
			.field("thread_id", &self.thread_id())
			.field("priority", &self.priority())
			.field("base_priority", &self.base_priority())
			.field("context_switches", &self.context_switches())
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
