use std::{ptr};
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use crate::winapi::*;
use crate::error::ErrorCode;
use crate::window::Window;
use crate::Result;

struct EnumWindowsContext<'a> {
	callback: &'a mut dyn FnMut(Window) -> bool,
}
#[allow(non_snake_case)]
unsafe extern "system" fn thunk(hwnd: HWND, lParam: LPARAM) -> BOOL {
	let context = &mut *(lParam as *mut EnumWindowsContext);
	// We are called from an FFI context so if this panics 'undefined behaviour' happens.
	// To solve this it should be wrapped in a `panic::catch_unwind` to catch panics but due to reasons I can't get this to work...
	// So fuck it! >.>
	if (context.callback)(Window(hwnd)) { TRUE }
	else { FALSE }
}

/// Enumerate all top-level windows.
///
/// See [EnumWindows function](https://msdn.microsoft.com/en-us/library/windows/desktop/ms633497.aspx) for more information.
pub fn windows<F>(mut f: F) -> bool where F: FnMut(Window) -> bool {
	let mut context = EnumWindowsContext {
		callback: &mut f,
	};
	unsafe {
		EnumWindows(Some(thunk), &mut context as *mut _ as LPARAM) != FALSE
	}
}

/// Find a window by class name or window title.
///
/// See [FindWindow function](https://msdn.microsoft.com/en-us/library/windows/desktop/ms633499.aspx) for more information.
pub fn find<T: AsRef<OsStr>>(class: Option<&T>, title: Option<&T>) -> Result<Window> {
	_find(class.map(|class| class.as_ref()), title.map(|title| title.as_ref()))
}
fn _find(class: Option<&OsStr>, title: Option<&OsStr>) -> Result<Window> {
	// These memory allocations make me cry...
	let class = class.map(|class| {
		let mut vec = class.encode_wide().collect::<Vec<u16>>();
		vec.push(0);
		vec
	});
	let title = title.map(|title| {
		let mut vec = title.encode_wide().collect::<Vec<u16>>();
		vec.push(0);
		vec
	});
	let wnd = unsafe {
		FindWindowW(
			class.map_or(ptr::null(), |class| class.as_ptr()),
			title.map_or(ptr::null(), |title| title.as_ptr()))
	};
	if wnd.is_null() {
		Err(ErrorCode::last())
	}
	else {
		Ok(Window(wnd))
	}
}
