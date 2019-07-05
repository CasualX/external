/*!
Low level mouse hook details.
!*/

use std::{ptr, fmt};
use crate::winapi::*;
use crate::vk::VirtualKey;
use super::HookContext;

//----------------------------------------------------------------

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MouseData {
	Move,
	ButtonDown(VirtualKey),
	ButtonUp(VirtualKey),
	DoubleClick(VirtualKey),
	Wheel(i16),
	HWheel(i16),
	Message,
}

/// Low level mouse hook callback context.
///
/// See documentation for
/// [LowLevelMouseProc](https://msdn.microsoft.com/en-us/library/windows/desktop/ms644986.aspx)
/// and
/// [MSLLHOOKSTRUCT](https://msdn.microsoft.com/en-us/library/windows/desktop/ms644970(v=vs.85).aspx)
/// for more information.
#[repr(C)]
pub struct MouseLL {
	code: c_int,
	message: u32,
	info: *mut MSLLHOOKSTRUCT,
	result: LRESULT,
}
impl MouseLL {
	pub fn cancel(&mut self) {
		self.result = !0;
	}

	pub fn message(&self) -> u32 {
		self.message
	}
	pub fn set_message(&mut self, message: u32) {
		self.message = message;
	}

	fn info_mut(&mut self) -> &mut MSLLHOOKSTRUCT {
		unsafe { &mut *self.info }
	}
	fn info(&self) -> &MSLLHOOKSTRUCT {
		unsafe { &*self.info }
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
	fn mouse_data_xbutton(&self) -> VirtualKey {
		match (self.info().mouseData >> 16) as u16 {
			XBUTTON1 => VirtualKey::XBUTTON1,
			XBUTTON2 => VirtualKey::XBUTTON2,
			_ => VirtualKey::NONE,
			// x => panic!("unknown xbutton: {}", x),
		}
	}
	fn mouse_data_wheel(&self) -> i16 {
		(self.info().mouseData >> 16) as i16
	}
	pub fn mouse_data(&self) -> MouseData {
		match self.message {
			WM_MOUSEMOVE => MouseData::Move,
			WM_LBUTTONDOWN => MouseData::ButtonDown(VirtualKey::LBUTTON),
			WM_LBUTTONUP => MouseData::ButtonUp(VirtualKey::LBUTTON),
			WM_RBUTTONDOWN => MouseData::ButtonDown(VirtualKey::RBUTTON),
			WM_RBUTTONUP => MouseData::ButtonUp(VirtualKey::RBUTTON),
			WM_XBUTTONDOWN => MouseData::ButtonDown(self.mouse_data_xbutton()),
			WM_XBUTTONUP => MouseData::ButtonUp(self.mouse_data_xbutton()),
			WM_MOUSEWHEEL => MouseData::Wheel(self.mouse_data_wheel()),
			WM_MOUSEHWHEEL => MouseData::HWheel(self.mouse_data_wheel()),
			_ => MouseData::Message,
		}
	}
	pub fn injected(&self) -> bool {
		self.info().flags & 0x01 != 0
	}
	pub fn set_injected(&mut self) {
		self.info_mut().flags |= 0x01;
	}
	pub fn clear_injected(&mut self) {
		self.info_mut().flags &= !0x01;
	}
	pub fn lower_il_injected(&self) -> bool {
		self.info().flags & 0x02 != 0
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
	pub unsafe fn extra_info_mut<T>(&mut self) -> Option<&mut T> {
		(self.info_mut().dwExtraInfo as *mut T).as_mut()
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
			.field("dwExtraInfo", &unsafe { (*self.info).dwExtraInfo as *const () })
			.finish()
	}
}
unsafe impl HookContext for MouseLL {
	fn hook_type() -> c_int {
		WH_MOUSE_LL
	}
	unsafe fn from_raw(code: c_int, w_param: WPARAM, l_param: LPARAM) -> Self {
		let message = w_param as u32;
		let info = l_param as *mut MSLLHOOKSTRUCT;
		MouseLL { code, message, info, result: 0 }
	}
	unsafe fn call_next_hook(&self) -> LRESULT {
		if self.result != 0 {
			self.result
		}
		else {
			let w_param = self.message as WPARAM;
			let l_param = self.info as LPARAM;
			CallNextHookEx(ptr::null_mut(), self.code, w_param, l_param)
		}
	}
}
