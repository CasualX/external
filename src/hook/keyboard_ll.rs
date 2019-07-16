/*!
Low level keyboard hook details.
!*/

use std::{ptr, fmt};
use crate::winapi::*;
use crate::vk::VirtualKey;
use super::HookContext;

//----------------------------------------------------------------

/// Low level keyboard hook callback context.
///
/// See documentation for
/// [LowLevelKeyboardProc](https://msdn.microsoft.com/en-us/library/windows/desktop/ms644985.aspx)
/// and
/// [KBDLLHOOKSTRUCT](https://msdn.microsoft.com/en-us/library/windows/desktop/ms644967.aspx)
/// for more information.
#[repr(C)]
pub struct KeyboardLL {
	code: c_int,
	message: u32,
	info: *mut KBDLLHOOKSTRUCT,
	result: LRESULT,
}
impl KeyboardLL {
	pub fn cancel(&mut self) {
		self.result = !0;
	}

	pub fn message(&self) -> u32 {
		self.message
	}
	pub fn set_message(&mut self, message: u32) {
		self.message = message;
	}

	fn info_mut(&mut self) -> &mut KBDLLHOOKSTRUCT {
		unsafe { &mut *self.info }
	}
	fn info(&self) -> &KBDLLHOOKSTRUCT {
		unsafe { &*self.info }
	}

	pub fn vk_code(&self) -> VirtualKey {
		self.info().vkCode.into()
	}
	pub fn set_vk_code(&mut self, vk_code: VirtualKey) {
		self.info_mut().vkCode = vk_code.into();
	}
	pub fn scan_code(&self) -> u32 {
		self.info().scanCode as u32
	}
	pub fn set_scan_code(&mut self, scan_code: u32) {
		self.info_mut().scanCode = scan_code;
	}
	pub fn extended(&self) -> bool {
		(self.info().flags & 0x01) != 0
	}
	pub fn set_extended(&mut self) {
		self.info_mut().flags |= 0x01;
	}
	pub fn clear_extended(&mut self) {
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
	pub fn injected(&self) -> bool {
		(self.info().flags & 0x10) != 0
	}
	pub fn set_injected(&mut self) {
		self.info_mut().flags |= 0x10;
	}
	pub fn clear_injected(&mut self) {
		self.info_mut().flags &= !0x10;
	}
	pub fn altdown(&self) -> bool {
		(self.info().flags & 0x20) != 0
	}
	pub fn set_altdown(&mut self) {
		self.info_mut().flags |= 0x20;
	}
	pub fn clear_altdown(&mut self) {
		self.info_mut().flags &= !0x20;
	}
	pub fn up(&self) -> bool {
		(self.info().flags & 0x80) != 0
	}
	pub fn set_up(&mut self) {
		self.info_mut().flags |= 0x80;
	}
	pub fn clear_up(&mut self) {
		self.info_mut().flags &= !0x80;
	}
	pub fn time(&self) -> u32 {
		self.info().time as u32
	}
	pub fn set_time(&mut self, time: u32) {
		self.info_mut().time = time;
	}

	pub unsafe fn extra_info<T>(&self) -> Option<&T> {
		(self.info().dwExtraInfo as *const T).as_ref()
	}
	pub unsafe fn extra_info_mut<T>(&mut self) -> Option<&mut T> {
		(self.info().dwExtraInfo as *mut T).as_mut()
	}
}
impl fmt::Debug for KeyboardLL {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("KeyboardLL")
			.field("message", &self.message())
			.field("vk_code", &self.vk_code())
			.field("scan_code", &self.scan_code())
			.field("extended", &self.extended())
			.field("lower_il_injected", &self.lower_il_injected())
			.field("injected", &self.injected())
			.field("altdown", &self.altdown())
			.field("up", &self.up())
			.field("time", &self.time())
			.field("dwExtraInfo", &(self.info().dwExtraInfo as *const ()))
			.finish()
	}
}
unsafe impl HookContext for KeyboardLL {
	fn hook_type() -> c_int {
		WH_KEYBOARD_LL
	}
	unsafe fn from_raw(code: c_int, w_param: WPARAM, l_param: LPARAM) -> Self {
		let message = w_param as u32;
		let info = l_param as *mut KBDLLHOOKSTRUCT;
		KeyboardLL { code, message, info, result: 0 }
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

//----------------------------------------------------------------

#[cfg(test)]
mod tests {
	use crate::wndclass::{pump_once};
	use crate::vk::{VirtualKey};

	#[test]
	fn test_keyboard_ll() {
		static mut PRESSED: bool = false;
		windows_hook! {
			pub fn my_callback(context: &mut super::KeyboardLL) {
				println!("{:#?}", context);
				if context.vk_code() == VirtualKey::SPACE {
					unsafe { PRESSED = true; }
				}
			}
		}
		let hook = my_callback().unwrap();
		VirtualKey::SPACE.down();
		VirtualKey::SPACE.up();
		pump_once();
		unsafe { assert_eq!(PRESSED, true); }
		drop(hook);
	}
}
