/*!
Low level mouse hook details.
*/

use std::{ptr, fmt};

use winapi::um::winuser::{SetWindowsHookExW, WH_MOUSE_LL, MSLLHOOKSTRUCT};
use winapi::um::winuser::{WM_MOUSEMOVE, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_RBUTTONDOWN, WM_RBUTTONUP, WM_XBUTTONDOWN, WM_XBUTTONUP, WM_MOUSEWHEEL, WM_MOUSEHWHEEL, XBUTTON1, XBUTTON2};
use winapi::shared::minwindef::{UINT, WPARAM};

use crate::error::ErrorCode;
use crate::input::{vk, VirtualKey};

use super::{Context, Invoke, Hook};

//----------------------------------------------------------------

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MouseData {
	Move,
	ButtonDown(VirtualKey),
	ButtonUp(VirtualKey),
	DoubleClick(VirtualKey),
	Wheel(i16),
	HWheel(i16),
}

/// Low level mouse hook callback context.
///
/// See documentation for
/// [LowLevelMouseProc](https://msdn.microsoft.com/en-us/library/windows/desktop/ms644986.aspx)
/// and
/// [MSLLHOOKSTRUCT](https://msdn.microsoft.com/en-us/library/windows/desktop/ms644970(v=vs.85).aspx)
/// for more information.
#[repr(C)]
pub struct MouseLL(Context);
impl MouseLL {
	pub fn cancel(&mut self) {
		self.0.result = !0;
	}

	pub fn message(&self) -> UINT {
		self.0.wParam as UINT
	}
	pub fn set_message(&mut self, message: UINT) {
		self.0.wParam = message as WPARAM;
	}

	fn info_mut(&mut self) -> &mut MSLLHOOKSTRUCT {
		unsafe { &mut *(self.0.lParam as *mut MSLLHOOKSTRUCT) }
	}
	fn info(&self) -> &MSLLHOOKSTRUCT {
		unsafe { &*(self.0.lParam as *const MSLLHOOKSTRUCT) }
	}

	pub fn pt_x(&self) -> i32 {
		self.info().pt.x
	}
	pub fn set_pt_x(&mut self, x: i32) {
		self.info_mut().pt.x = x;
	}
	pub fn pt_y(&self) -> i32 {
		self.info().pt.y
	}
	pub fn set_pt_y(&mut self, y: i32) {
		self.info_mut().pt.y = y;
	}
	pub fn mouse_data(&self) -> MouseData {
		match self.0.wParam as UINT {
			WM_MOUSEMOVE => MouseData::Move,
			WM_LBUTTONDOWN => MouseData::ButtonDown(vk::LBUTTON),
			WM_LBUTTONUP => MouseData::ButtonUp(vk::LBUTTON),
			WM_RBUTTONDOWN => MouseData::ButtonDown(vk::RBUTTON),
			WM_RBUTTONUP => MouseData::ButtonUp(vk::RBUTTON),
			message @ WM_XBUTTONDOWN ... WM_XBUTTONUP => {
				let vk = match (self.info().mouseData >> 16) as u16 {
					XBUTTON1 => vk::XBUTTON1,
					XBUTTON2 => vk::XBUTTON2,
					button => panic!("unknown xbutton: {}", button),
				};
				if message == WM_XBUTTONDOWN {
					MouseData::ButtonDown(vk)
				}
				else {
					MouseData::ButtonUp(vk)
				}
			},
			WM_MOUSEWHEEL => MouseData::Wheel((self.info().mouseData >> 16) as i16),
			WM_MOUSEHWHEEL => MouseData::HWheel((self.info().mouseData >> 16) as i16),
			message => panic!("unexpected message: {:#X}", message),
		}
	}
	pub fn injected(&self) -> bool {
		(self.info().flags & 0x01) != 0
	}
	pub fn set_injected(&mut self) {
		self.info_mut().flags |= 0x01;
	}
	pub fn clear_injected(&mut self) {
		self.info_mut().flags &= !0x01;
	}
	pub fn lower_il_injected(&self) -> bool {
		(self.info().flags & 0x02) != 0
	}
	pub fn set_lower_il_injected(&mut self) {
		self.info_mut().flags |= 0x02;
	}
	pub fn clear_lower_il_injected(&mut self) {
		self.info_mut().flags &= !0x02;
	}
	pub fn time(&self) -> u32 {
		self.info().time
	}
	pub fn set_time(&mut self, time: u32) {
		self.info_mut().time = time;
	}
	pub unsafe fn extra_info<T>(&self) -> Option<&T> {
		(self.info().dwExtraInfo as *const T).as_ref()
	}
	pub unsafe fn extra_info_mut<T>(&self) -> Option<&mut T> {
		(self.info().dwExtraInfo as *mut T).as_mut()
	}
}
impl fmt::Debug for MouseLL {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("MouseLL")
			.field("message", &self.message())
			.field("pt_x", &self.pt_x())
			.field("pt_y", &self.pt_y())
			.field("mouse_data", &self.mouse_data())
			.field("injected", &self.injected())
			.field("lower_il_injected", &self.lower_il_injected())
			.field("time", &self.time())
			.field("dwExtraInfo", &(self.info().dwExtraInfo as *const ()))
			.finish()
	}
}

/// Low level mouse hook callback.
pub trait CallMouseLL: Invoke {
	fn callback(arg: &mut MouseLL);
	/// Registers the low-level mouse hook.
	fn register() -> Result<Hook, ErrorCode> {
		unsafe {
			let hook = SetWindowsHookExW(WH_MOUSE_LL, Some(Self::thunk), ptr::null_mut(), 0);
			if hook.is_null() {
				Err(ErrorCode::last())
			}
			else {
				Ok(Hook(hook))
			}
		}
	}
}
