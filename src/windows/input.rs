/*!
Virtual keys and input.
*/

use winapi::{c_int, DWORD};

/// Windows virtual key code.
///
/// See [Virtual-Key Codes](https://msdn.microsoft.com/en-us/library/windows/desktop/dd375731.aspx) for more information.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct VirtualKey(u8);
impl From<DWORD> for VirtualKey {
	fn from(vkey: DWORD) -> VirtualKey {
		VirtualKey(vkey as u8)
	}
}
impl From<c_int> for VirtualKey {
	fn from(vkey: c_int) -> VirtualKey {
		VirtualKey(vkey as u8)
	}
}
impl From<VirtualKey> for DWORD {
	fn from(vkey: VirtualKey) -> DWORD {
		vkey.0 as DWORD
	}
}
impl From<VirtualKey> for BYTE {
	fn from(vkey: VirtualKey) -> BYTE {
		vkey.0 as BYTE
	}
}
/// VirtualKey constants.
///
/// Whofully incomplete, please finish me...
pub mod vk {
	use super::VirtualKey;

	pub const LBUTTON: VirtualKey = VirtualKey(::winapi::VK_LBUTTON as u8);
	pub const RBUTTON: VirtualKey = VirtualKey(::winapi::VK_RBUTTON as u8);
	pub const CANCEL: VirtualKey = VirtualKey(::winapi::VK_CANCEL as u8);
	pub const MBUTTON: VirtualKey = VirtualKey(::winapi::VK_MBUTTON as u8);
	pub const XBUTTON1: VirtualKey = VirtualKey(::winapi::VK_XBUTTON1 as u8);
	pub const XBUTTON2: VirtualKey = VirtualKey(::winapi::VK_XBUTTON2 as u8);
	pub const BACK: VirtualKey = VirtualKey(::winapi::VK_BACK as u8);
	pub const TAB: VirtualKey = VirtualKey(::winapi::VK_TAB as u8);
	pub const CLEAR: VirtualKey = VirtualKey(::winapi::VK_CLEAR as u8);
	pub const RETURN: VirtualKey = VirtualKey(::winapi::VK_RETURN as u8);

	pub const SHIFT: VirtualKey = VirtualKey(::winapi::VK_SHIFT as u8);
	pub const CONTROL: VirtualKey = VirtualKey(::winapi::VK_CONTROL as u8);
	pub const ALT: VirtualKey = VirtualKey(::winapi::VK_MENU as u8);
	pub const PAUSE: VirtualKey = VirtualKey(::winapi::VK_PAUSE as u8);
	pub const CAPSLOCK: VirtualKey = VirtualKey(::winapi::VK_CAPITAL as u8);
	pub const ESCAPE: VirtualKey = VirtualKey(::winapi::VK_ESCAPE as u8);

	pub const SPACE: VirtualKey = VirtualKey(::winapi::VK_SPACE as u8);
	pub const PAGE_UP: VirtualKey = VirtualKey(::winapi::VK_PRIOR as u8);
	pub const PAGE_DOWN: VirtualKey = VirtualKey(::winapi::VK_NEXT as u8);
	pub const END: VirtualKey = VirtualKey(::winapi::VK_END as u8);
	pub const HOME: VirtualKey = VirtualKey(::winapi::VK_HOME as u8);
	pub const LEFT: VirtualKey = VirtualKey(::winapi::VK_LEFT as u8);
	pub const UP: VirtualKey = VirtualKey(::winapi::VK_UP as u8);
	pub const RIGHT: VirtualKey = VirtualKey(::winapi::VK_RIGHT as u8);
	pub const DOWN: VirtualKey = VirtualKey(::winapi::VK_DOWN as u8);
	pub const SNAPSHOT: VirtualKey = VirtualKey(::winapi::VK_SNAPSHOT as u8);
	pub const INSERT: VirtualKey = VirtualKey(::winapi::VK_INSERT as u8);
	pub const DELETE: VirtualKey = VirtualKey(::winapi::VK_DELETE as u8);

	//pub const : VirtualKey = VirtualKey(::winapi::VK_ as u8);
}

//----------------------------------------------------------------

use ::user32::{keybd_event, mouse_event, MapVirtualKeyW, GetAsyncKeyState, GetSystemMetrics};
use ::winapi::{BYTE, MAPVK_VK_TO_VSC, KEYEVENTF_KEYUP, XBUTTON1, XBUTTON2, SM_CXSCREEN, SM_CYSCREEN};
use ::winapi::{MOUSEEVENTF_MOVE, MOUSEEVENTF_ABSOLUTE, MOUSEEVENTF_WHEEL,
	MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP, 
	MOUSEEVENTF_MIDDLEDOWN, MOUSEEVENTF_MIDDLEUP, MOUSEEVENTF_XDOWN, MOUSEEVENTF_XUP};

/// Press a virtual key.
pub fn key_down(vkey: VirtualKey) {
	key_send(vkey, false);
}
/// Release a virtual key.
pub fn key_up(vkey: VirtualKey) {
	key_send(vkey, true);
}

fn key_send(vkey: VirtualKey, up: bool) {
	unsafe {
		if vkey == vk::LBUTTON {
			mouse_event(if up { MOUSEEVENTF_LEFTUP } else { MOUSEEVENTF_LEFTDOWN }, 0, 0, 0, 0);
		}
		else if vkey == vk::RBUTTON {
			mouse_event(if up { MOUSEEVENTF_RIGHTUP } else { MOUSEEVENTF_RIGHTDOWN }, 0, 0, 0, 0);
		}
		else if vkey == vk::MBUTTON {
			mouse_event(if up { MOUSEEVENTF_MIDDLEUP } else { MOUSEEVENTF_MIDDLEDOWN }, 0, 0, 0, 0);
		}
		else if vkey == vk::XBUTTON1 {
			mouse_event(if up { MOUSEEVENTF_XUP } else { MOUSEEVENTF_XDOWN }, 0, 0, XBUTTON1, 0);
		}
		else if vkey == vk::XBUTTON2 {
			mouse_event(if up { MOUSEEVENTF_XUP } else { MOUSEEVENTF_XDOWN }, 0, 0, XBUTTON2, 0);
		}
		else {
			let scan_code = MapVirtualKeyW(vkey.into(), MAPVK_VK_TO_VSC);
			keybd_event(vkey.into(), scan_code as BYTE, if up { KEYEVENTF_KEYUP } else { 0 }, 0);
		}
	}
}

pub fn async_key_state(vkey: VirtualKey) -> bool {
	unsafe {
		(GetAsyncKeyState(vkey.0 as i32) as u16 & 0x8000) != 0
	}
}

/// Move the mouse relatively.
pub fn mouse_move(dx: i32, dy: i32) {
	unsafe {
		mouse_event(MOUSEEVENTF_MOVE, dx as DWORD, dy as DWORD, 0, 0);
	}
}
/// Set the mouse position in absolute pixel coordinates.
pub fn mouse_set(dx: u32, dy: u32) {
	unsafe {
		mouse_event(MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE, dx as DWORD, dy as DWORD, 0, 0);
	}
}
/// Scroll the mouse wheel.
pub fn mouse_wheel(delta: i32) {
	unsafe {
		mouse_event(MOUSEEVENTF_WHEEL, 0, 0, delta as DWORD, 0);
	}
}

pub fn primary_screen_size() -> (i32, i32) {
	unsafe {
		let width = GetSystemMetrics(SM_CXSCREEN);
		let height = GetSystemMetrics(SM_CYSCREEN);
		(width, height)
	}
}
