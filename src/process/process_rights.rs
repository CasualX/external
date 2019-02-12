use crate::winapi::*;

/// Create process access rights using the builder pattern.
///
/// See [Process Security and Access Rights](https://msdn.microsoft.com/en-us/library/windows/desktop/ms684880.aspx) for more information.
pub struct ProcessRights(DWORD);
impl_inner!(ProcessRights: DWORD);
impl ProcessRights {
	pub fn new() -> ProcessRights {
		ProcessRights(0)
	}
	pub const ALL_ACCESS: ProcessRights = ProcessRights(PROCESS_ALL_ACCESS);

	pub fn delete(self) -> ProcessRights {
		ProcessRights(self.0 | DELETE)
	}
	pub fn read_control(self) -> ProcessRights {
		ProcessRights(self.0 | READ_CONTROL)
	}
	pub fn synchronize(self) -> ProcessRights {
		ProcessRights(self.0 | SYNCHRONIZE)
	}
	pub fn write_dac(self) -> ProcessRights {
		ProcessRights(self.0 | WRITE_DAC)
	}
	pub fn write_owner(self) -> ProcessRights {
		ProcessRights(self.0 | WRITE_OWNER)
	}

	pub fn create_process(self) -> ProcessRights {
		ProcessRights(self.0 | PROCESS_CREATE_PROCESS)
	}
	pub fn create_thread(self) -> ProcessRights {
		ProcessRights(self.0 | PROCESS_CREATE_THREAD)
	}
	pub fn dup_handle(self) -> ProcessRights {
		ProcessRights(self.0 | PROCESS_DUP_HANDLE)
	}
	pub fn query_information(self) -> ProcessRights {
		ProcessRights(self.0 | PROCESS_QUERY_INFORMATION)
	}
	pub fn query_limited_information(self) -> ProcessRights {
		ProcessRights(self.0 | PROCESS_QUERY_LIMITED_INFORMATION)
	}
	pub fn set_information(self) -> ProcessRights {
		ProcessRights(self.0 | PROCESS_SET_INFORMATION)
	}
	pub fn set_quota(self) -> ProcessRights {
		ProcessRights(self.0 | PROCESS_SET_QUOTA)
	}
	pub fn suspend_resume(self) -> ProcessRights {
		ProcessRights(self.0 | PROCESS_SUSPEND_RESUME)
	}
	pub fn terminate(self) -> ProcessRights {
		ProcessRights(self.0 | PROCESS_TERMINATE)
	}
	pub fn vm_operation(self) -> ProcessRights {
		ProcessRights(self.0 | PROCESS_VM_OPERATION)
	}
	pub fn vm_read(self) -> ProcessRights {
		ProcessRights(self.0 | PROCESS_VM_READ)
	}
	pub fn vm_write(self) -> ProcessRights {
		ProcessRights(self.0 | PROCESS_VM_WRITE)
	}
}
