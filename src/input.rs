/*!
Virtual keys and input.
!*/

use crate::winapi::*;

/// Windows virtual key code.
///
/// See [Virtual-Key Codes](https://msdn.microsoft.com/en-us/library/windows/desktop/dd375731.aspx) for more information.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
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
impl From<BYTE> for VirtualKey {
	fn from(vkey: BYTE) -> VirtualKey {
		VirtualKey(vkey)
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
impl VirtualKey {
	/// Is this the `NONE` key.
	pub fn is_none(self) -> bool {
		self.0 == 0
	}
	/// Is this a mouse button.
	pub fn is_mouse(self) -> bool {
		self.0 < 32 && (1 << self.0 as u32) & 0x76 != 0
	}
	/// Is this a keyboard key.
	pub fn is_keybd(self) -> bool {
		!(self.0 < 32 && (1 << self.0 as u32) & 0x77 != 0)
	}
}
/// VirtualKey constants.
impl VirtualKey {
	pub const NONE: VirtualKey = VirtualKey(0);

	pub const LBUTTON: VirtualKey = VirtualKey(winapi::um::winuser::VK_LBUTTON as u8);
	pub const RBUTTON: VirtualKey = VirtualKey(winapi::um::winuser::VK_RBUTTON as u8);
	pub const CANCEL: VirtualKey = VirtualKey(winapi::um::winuser::VK_CANCEL as u8);
	pub const MBUTTON: VirtualKey = VirtualKey(winapi::um::winuser::VK_MBUTTON as u8);
	pub const XBUTTON1: VirtualKey = VirtualKey(winapi::um::winuser::VK_XBUTTON1 as u8);
	pub const XBUTTON2: VirtualKey = VirtualKey(winapi::um::winuser::VK_XBUTTON2 as u8);
	pub const BACK: VirtualKey = VirtualKey(winapi::um::winuser::VK_BACK as u8);
	pub const TAB: VirtualKey = VirtualKey(winapi::um::winuser::VK_TAB as u8);
	pub const CLEAR: VirtualKey = VirtualKey(winapi::um::winuser::VK_CLEAR as u8);
	pub const RETURN: VirtualKey = VirtualKey(winapi::um::winuser::VK_RETURN as u8);

	pub const SHIFT: VirtualKey = VirtualKey(winapi::um::winuser::VK_SHIFT as u8);
	pub const CONTROL: VirtualKey = VirtualKey(winapi::um::winuser::VK_CONTROL as u8);
	pub const ALT: VirtualKey = VirtualKey(winapi::um::winuser::VK_MENU as u8);
	pub const PAUSE: VirtualKey = VirtualKey(winapi::um::winuser::VK_PAUSE as u8);
	pub const CAPSLOCK: VirtualKey = VirtualKey(winapi::um::winuser::VK_CAPITAL as u8);
	pub const ESCAPE: VirtualKey = VirtualKey(winapi::um::winuser::VK_ESCAPE as u8);

	pub const SPACE: VirtualKey = VirtualKey(winapi::um::winuser::VK_SPACE as u8);
	pub const PAGE_UP: VirtualKey = VirtualKey(winapi::um::winuser::VK_PRIOR as u8);
	pub const PAGE_DOWN: VirtualKey = VirtualKey(winapi::um::winuser::VK_NEXT as u8);
	pub const END: VirtualKey = VirtualKey(winapi::um::winuser::VK_END as u8);
	pub const HOME: VirtualKey = VirtualKey(winapi::um::winuser::VK_HOME as u8);
	pub const LEFT: VirtualKey = VirtualKey(winapi::um::winuser::VK_LEFT as u8);
	pub const UP: VirtualKey = VirtualKey(winapi::um::winuser::VK_UP as u8);
	pub const RIGHT: VirtualKey = VirtualKey(winapi::um::winuser::VK_RIGHT as u8);
	pub const DOWN: VirtualKey = VirtualKey(winapi::um::winuser::VK_DOWN as u8);
	pub const SNAPSHOT: VirtualKey = VirtualKey(winapi::um::winuser::VK_SNAPSHOT as u8);
	pub const INSERT: VirtualKey = VirtualKey(winapi::um::winuser::VK_INSERT as u8);
	pub const DELETE: VirtualKey = VirtualKey(winapi::um::winuser::VK_DELETE as u8);

	//pub const : VirtualKey = VirtualKey(winapi::um::winuser::VK_ as u8);
}
impl VirtualKey {
	/// Press a virtual key.
	pub fn down(self) {
		self.send(false);
	}
	/// Release a virtual key.
	pub fn up(self) {
		self.send(true);
	}
	fn send(self, up: bool) {
		unsafe {
			match self {
				VirtualKey::LBUTTON => mouse_event(if up { MOUSEEVENTF_LEFTUP } else { MOUSEEVENTF_LEFTDOWN }, 0, 0, 0, 0),
				VirtualKey::RBUTTON => mouse_event(if up { MOUSEEVENTF_RIGHTUP } else { MOUSEEVENTF_RIGHTDOWN }, 0, 0, 0, 0),
				VirtualKey::MBUTTON => mouse_event(if up { MOUSEEVENTF_MIDDLEUP } else { MOUSEEVENTF_MIDDLEDOWN }, 0, 0, 0, 0),
				VirtualKey::XBUTTON1 => mouse_event(if up { MOUSEEVENTF_XUP } else { MOUSEEVENTF_XDOWN }, 0, 0, XBUTTON1 as DWORD, 0),
				VirtualKey::XBUTTON2 => mouse_event(if up { MOUSEEVENTF_XUP } else { MOUSEEVENTF_XDOWN }, 0, 0, XBUTTON2 as DWORD, 0),
				vkey => keybd_event(vkey.into(), MapVirtualKeyW(vkey.into(), MAPVK_VK_TO_VSC) as BYTE, if up { KEYEVENTF_KEYUP } else { 0 }, 0),
			}
		}
	}
	/// Gets the async key state.
	pub fn async_state(self) -> bool {
		unsafe {
			(GetAsyncKeyState(self.0 as i32) as u16 & 0x8000) != 0
		}
	}
}

#[test]
fn test_key_types() {
	assert!(VirtualKey::NONE.is_none());
	assert!(!VirtualKey::NONE.is_mouse());
	assert!(VirtualKey::LBUTTON.is_mouse());
	assert!(VirtualKey::RBUTTON.is_mouse());
	assert!(VirtualKey::MBUTTON.is_mouse());
	assert!(VirtualKey::XBUTTON1.is_mouse());
	assert!(VirtualKey::XBUTTON2.is_mouse());
	assert!(!VirtualKey::NONE.is_keybd());
	assert!(!VirtualKey::LBUTTON.is_keybd());
	assert!(!VirtualKey::RBUTTON.is_keybd());
	assert!(!VirtualKey::MBUTTON.is_keybd());
	assert!(!VirtualKey::XBUTTON1.is_keybd());
	assert!(!VirtualKey::XBUTTON2.is_keybd());
}

//----------------------------------------------------------------

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
