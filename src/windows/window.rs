/*!
Window handles.
*/

use std::{mem, ptr};
use std::ffi::{OsString, OsStr};
use std::os::windows::ffi::{OsStringExt, OsStrExt};

use winapi::{GWLP_USERDATA};
use user32::{IsWindow, ShowWindow, UpdateWindow, GetWindowTextW,
	GetWindowThreadProcessId,
	RealGetWindowClassW, EnumWindows, FindWindowW, GetForegroundWindow,
	GetClientRect, ClientToScreen, ScreenToClient, GetDesktopWindow};
use winapi::{BOOL, FALSE, TRUE, DWORD, LONG_PTR, WCHAR, HWND, LPARAM, POINT};

use process::ProcessId;
use thread::ThreadId;
use error::ErrorCode;
use {Result, FromInner, IntoInner};

/// Abstracts a `HWND`.
///
/// This is slightly special because `HWND` has no concept of ownership or anything so this abstraction doesn't try to create one.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Window(HWND);
impl_inner!(Window: HWND);
impl Window {
	/// Get the foreground window.
	///
	/// See [GetForegroundWindow function](https://msdn.microsoft.com/en-us/library/windows/desktop/ms633505.aspx) for more information.
	pub fn foreground() -> Option<Window> {
		unsafe {
			let hwnd = GetForegroundWindow();
			if hwnd.is_null() { None }
			else { Some(Window(hwnd)) }
		}
	}
	/// Get the desktop window.
	///
	/// See [GetDesktopWindow function](https://msdn.microsoft.com/en-us/library/windows/desktop/ms633504.aspx) for more information.
	pub fn desktop() -> Window {
		unsafe {
			Window(GetDesktopWindow())
		}
	}
	pub fn null() -> Window {
		Window(ptr::null_mut())
	}
	/// Returns if this window is still valid.
	pub fn valid(self) -> bool {
		unsafe {
			IsWindow(self.into_inner()) != FALSE
		}
	}
	/// Returns the class name of this window.
	pub fn class(self) -> Result<OsString> {
		unsafe {
			// 260 ought to be enough for everyone.
			let mut buf: [WCHAR; 260] = mem::uninitialized();
			let len = RealGetWindowClassW(self.into_inner(), buf.as_mut_ptr(), 260);
			if len == 0 {
				Err(ErrorCode::last())
			}
			else {
				Ok(OsString::from_wide(&buf[..len as usize]))
			}
		}
	}
	pub fn show(self, cmd: i32) {
		unsafe {
			ShowWindow(self.into_inner(), cmd);
		}
	}
	pub fn update(self) -> Result<()> {
		unsafe {
			if UpdateWindow(self.into_inner()) == FALSE {
				Err(ErrorCode::last())
			}
			else {
				Ok(())
			}
		}
	}
	#[cfg(target_pointer_width = "64")]
	pub fn user_data(self) -> usize {
		unsafe {
			::user32::GetWindowLongPtrW(self.into_inner(), GWLP_USERDATA) as usize
		}
	}
	#[cfg(target_pointer_width = "64")]
	pub fn set_user_data<T>(self, data: usize) {
		unsafe {
			::user32::SetWindowLongPtrW(self.into_inner(), GWLP_USERDATA, data as LONG_PTR);
		}
	}
	#[cfg(target_pointer_width = "32")]
	pub fn user_data(self) -> usize {
		unsafe {
			::user32::GetWindowLongW(self.into_inner(), GWLP_USERDATA) as usize
		}
	}
	#[cfg(target_pointer_width = "32")]
	pub fn set_user_data<T>(self, data: usize) {
		unsafe {
			::user32::SetWindowLongW(self.into_inner(), GWLP_USERDATA, data as LONG_PTR);
		}
	}
	/// Returns the window title of this window.
	pub fn title(self) -> Result<OsString> {
		unsafe {
			// 260 ought to be enough for everyone.
			let mut buf: [WCHAR; 260] = mem::uninitialized();
			let len = GetWindowTextW(self.into_inner(), buf.as_mut_ptr(), 260);
			if len <= 0 {
				Err(ErrorCode::last())
			}
			else {
				Ok(OsString::from_wide(&buf[..len as usize]))
			}
		}
	}
	/// Returns the thread and process id associated with this window.
	pub fn thread_process_id(self) -> (ThreadId, ProcessId) {
		unsafe {
			let mut process_id: DWORD = mem::uninitialized();
			let thread_id = GetWindowThreadProcessId(self.into_inner(), &mut process_id);
			(ThreadId::from_inner(thread_id), ProcessId::from_inner(process_id))
		}
	}
	/// Retrieves the coordinates of a window's client area.
	pub fn client_area(self) -> Result<(i32, i32)> {
		unsafe {
			let mut rc = mem::uninitialized();
			if GetClientRect(self.into_inner(), &mut rc) == FALSE {
				Err(ErrorCode::last())
			}
			else {
				Ok((rc.right, rc.bottom))
			}
		}
	}
	/// Convert the client-area coordinates of a specified point to screen coordinates.
	///
	/// See [ClientToScreen function](https://msdn.microsoft.com/en-us/library/vs/alm/dd183434.aspx) for more information.
	pub fn client_to_screen(self, point: (i32, i32)) -> Result<(i32, i32)> {
		unsafe {
			let mut pt = POINT { x: point.0, y: point.1 };
			if ClientToScreen(self.into_inner(), &mut pt) == FALSE {
				Err(ErrorCode::last())
			}
			else {
				Ok((pt.x, pt.y))
			}
		}
	}
	/// Convert the screen coordinates of a specified point on the screen to client-area coordinates.
	///
	/// See [ScreenToClient](https://msdn.microsoft.com/en-us/library/vs/alm/dd162952.aspx) for more information.
	pub fn screen_to_client(self, point: (i32, i32)) -> Result<(i32, i32)> {
		unsafe {
			let mut pt = POINT { x: point.0, y: point.1 };
			if ScreenToClient(self.0, &mut pt) == FALSE {
				Err(ErrorCode::last())
			}
			else {
				Ok((pt.x, pt.y))
			}
		}
	}
}

//----------------------------------------------------------------

struct EnumWindowsContext<'a> {
	callback: &'a mut FnMut(Window) -> bool,
}
#[allow(non_snake_case)]
unsafe extern "system" fn thunk(hwnd: HWND, lParam: LPARAM) -> BOOL {
	let mut context = &mut *(lParam as *mut EnumWindowsContext);
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
