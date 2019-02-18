use crate::winapi::*;

//use winapi::{THREAD_ALL_ACCESS, THREAD_DIRECT_IMPERSONATION, THREAD_GET_CONTEXT, THREAD_IMPERSONATE, THREAD_QUERY_INFORMATION, THREAD_QUERY_LIMITED_INFORMATION,
//	THREAD_SET_CONTEXT, THREAD_SET_INFORMATION, THREAD_SET_LIMITED_INFORMATION, THREAD_SET_THREAD_TOKEN, THREAD_SUSPEND_RESUME, THREAD_TERMINATE};

/// Create thread access rights using the builder pattern.
///
/// See [Thread Security and Access Rights](https://msdn.microsoft.com/en-us/library/windows/desktop/ms686769.aspx) for more information.
pub struct ThreadRights(DWORD);
impl_inner!(ThreadRights: DWORD);
impl ThreadRights {
	pub fn new() -> ThreadRights {
		ThreadRights(0)
	}
	pub fn all_access() -> ThreadRights {
		ThreadRights(0x1F0FFB)
	}

	pub fn delete(self) -> ThreadRights {
		ThreadRights(self.0 | DELETE)
	}
	pub fn read_control(self) -> ThreadRights {
		ThreadRights(self.0 | READ_CONTROL)
	}
	pub fn synchronize(self) -> ThreadRights {
		ThreadRights(self.0 | SYNCHRONIZE)
	}
	pub fn write_dac(self) -> ThreadRights {
		ThreadRights(self.0 | WRITE_DAC)
	}
	pub fn write_owner(self) -> ThreadRights {
		ThreadRights(self.0 | WRITE_OWNER)
	}

	pub fn direct_impersonation(self) -> ThreadRights {
		ThreadRights(self.0 | 0x0200)
	}
	pub fn get_context(self) -> ThreadRights {
		ThreadRights(self.0 | 0x0008)
	}
	pub fn impersonate(self) -> ThreadRights {
		ThreadRights(self.0 | 0x0100)
	}
	pub fn query_information(self) -> ThreadRights {
		ThreadRights(self.0 | 0x0040)
	}
	pub fn query_limited_information(self) -> ThreadRights {
		ThreadRights(self.0 | 0x0800)
	}
	pub fn set_context(self) -> ThreadRights {
		ThreadRights(self.0 | 0x0010)
	}
	pub fn set_information(self) -> ThreadRights {
		ThreadRights(self.0 | 0x0020)
	}
	pub fn set_limited_information(self) -> ThreadRights {
		ThreadRights(self.0 | 0x0400)
	}
	pub fn set_thread_token(self) -> ThreadRights {
		ThreadRights(self.0 | 0x0080)
	}
	pub fn suspend_resume(self) -> ThreadRights {
		ThreadRights(self.0 | 0x0002)
	}
	pub fn terminate(self) -> ThreadRights {
		ThreadRights(self.0 | 0x0001)
	}
}
