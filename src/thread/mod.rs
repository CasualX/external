/*!
Threads.
!*/

mod thread_id;
mod thread_rights;
mod thread_enum;
// mod thread_tib;
mod thread;

pub use self::thread_id::*;
pub use self::thread_rights::*;
pub use self::thread_enum::*;
// pub use self::thread_tib::*;
pub use self::thread::*;

/// CreateThread from DllMain and calls FreeLibraryAndExitThread when the function returns.
///
/// The purpose of this function is to aid injected DLLs to start a new thread from inside DllMain's DLL_PROCESS_ATTACH event and free themselves when exited.
pub unsafe fn start(f: fn()) {
		use std::{mem, ptr};
		use crate::winapi::*;
		extern "system" fn thunk(f: LPVOID) -> DWORD {
			unsafe {
				mem::transmute::<_, fn()>(f)();
				FreeLibraryAndExitThread(crate::module::image_base(), 0);
			}
			return 0;
		}
		CreateThread(ptr::null_mut(), 0, Some(thunk), f as LPVOID, 0, ptr::null_mut());
}
