use crate::winapi::*;

/// Create process access rights using the builder pattern.
///
/// See [Process Security and Access Rights](https://msdn.microsoft.com/en-us/library/windows/desktop/ms684880.aspx) for more information.
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct ProcessRights(DWORD);
impl_inner!(ProcessRights: safe DWORD);
impl ProcessRights {
	pub const fn new() -> ProcessRights {
		ProcessRights(0)
	}
	pub const ALL_ACCESS: ProcessRights = ProcessRights(PROCESS_ALL_ACCESS);

	pub const fn delete(self) -> ProcessRights {
		ProcessRights(self.0 | DELETE)
	}
	pub const fn read_control(self) -> ProcessRights {
		ProcessRights(self.0 | READ_CONTROL)
	}
	pub const fn synchronize(self) -> ProcessRights {
		ProcessRights(self.0 | SYNCHRONIZE)
	}
	pub const fn write_dac(self) -> ProcessRights {
		ProcessRights(self.0 | WRITE_DAC)
	}
	pub const fn write_owner(self) -> ProcessRights {
		ProcessRights(self.0 | WRITE_OWNER)
	}

	pub const fn create_process(self) -> ProcessRights {
		ProcessRights(self.0 | PROCESS_CREATE_PROCESS)
	}
	pub const fn create_thread(self) -> ProcessRights {
		ProcessRights(self.0 | PROCESS_CREATE_THREAD)
	}
	pub const fn dup_handle(self) -> ProcessRights {
		ProcessRights(self.0 | PROCESS_DUP_HANDLE)
	}
	pub const fn query_information(self) -> ProcessRights {
		ProcessRights(self.0 | PROCESS_QUERY_INFORMATION)
	}
	pub const fn query_limited_information(self) -> ProcessRights {
		ProcessRights(self.0 | PROCESS_QUERY_LIMITED_INFORMATION)
	}
	pub const fn set_information(self) -> ProcessRights {
		ProcessRights(self.0 | PROCESS_SET_INFORMATION)
	}
	pub const fn set_quota(self) -> ProcessRights {
		ProcessRights(self.0 | PROCESS_SET_QUOTA)
	}
	pub const fn suspend_resume(self) -> ProcessRights {
		ProcessRights(self.0 | PROCESS_SUSPEND_RESUME)
	}
	pub const fn terminate(self) -> ProcessRights {
		ProcessRights(self.0 | PROCESS_TERMINATE)
	}
	pub const fn vm_operation(self) -> ProcessRights {
		ProcessRights(self.0 | PROCESS_VM_OPERATION)
	}
	pub const fn vm_read(self) -> ProcessRights {
		ProcessRights(self.0 | PROCESS_VM_READ)
	}
	pub const fn vm_write(self) -> ProcessRights {
		ProcessRights(self.0 | PROCESS_VM_WRITE)
	}
}
