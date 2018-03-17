use std::{mem, ptr};
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

use winapi::um::winuser::*;
use winapi::shared::basetsd::{LONG_PTR};
use winapi::shared::ntdef::{WCHAR};
use winapi::shared::windef::{HWND, POINT};
use winapi::shared::minwindef::{FALSE, DWORD};

use process::ProcessId;
use thread::ThreadId;
use error::ErrorCode;
use {Result, FromInner, IntoInner};

/// Abstracts a `HWND`.
///
/// This is slightly special because `HWND` has no concept of ownership or anything so this abstraction doesn't try to create one.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Window(pub(super) HWND);
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
			GetWindowLongPtrW(self.into_inner(), GWLP_USERDATA) as usize
		}
	}
	#[cfg(target_pointer_width = "64")]
	pub fn set_user_data<T>(self, data: usize) {
		unsafe {
			SetWindowLongPtrW(self.into_inner(), GWLP_USERDATA, data as LONG_PTR);
		}
	}
	#[cfg(target_pointer_width = "32")]
	pub fn user_data(self) -> usize {
		unsafe {
			GetWindowLongW(self.into_inner(), GWLP_USERDATA) as usize
		}
	}
	#[cfg(target_pointer_width = "32")]
	pub fn set_user_data<T>(self, data: usize) {
		unsafe {
			SetWindowLongW(self.into_inner(), GWLP_USERDATA, data as LONG_PTR);
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
