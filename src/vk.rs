/*!
Virtual keys.
!*/

use crate::winapi::*;

/// Windows virtual key code.
///
/// See [Virtual-Key Codes](https://msdn.microsoft.com/en-us/library/windows/desktop/dd375731.aspx) for more information.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct VirtualKey(u8);
impl_inner!(VirtualKey: safe u8);
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
impl VirtualKey {
	/// VirtualKey const constructor.
	#[inline]
	pub const fn new(vk: u8) -> VirtualKey {
		VirtualKey(vk)
	}
	/// Is this the `NONE` key.
	#[inline]
	pub const fn is_none(self) -> bool {
		self.0 == 0
	}
	/// Is this a mouse button.
	#[inline]
	pub const fn is_mouse(self) -> bool {
		(self.0 < 32) & ((1 << self.0 as u32) & 0x76 != 0)
	}
	/// Is this a keyboard key.
	#[inline]
	pub const fn is_keybd(self) -> bool {
		!((self.0 < 32) & ((1 << self.0 as u32) & 0x77 != 0))
	}
}
/// VirtualKey constants.
impl VirtualKey {
	pub const NONE: VirtualKey = VirtualKey(0x00);

	pub const LBUTTON: VirtualKey = VirtualKey(0x01);
	pub const RBUTTON: VirtualKey = VirtualKey(0x02);
	pub const CANCEL: VirtualKey = VirtualKey(0x03);
	pub const MBUTTON: VirtualKey = VirtualKey(0x04);
	pub const XBUTTON1: VirtualKey = VirtualKey(0x05);
	pub const XBUTTON2: VirtualKey = VirtualKey(0x06);
	pub const BACK: VirtualKey = VirtualKey(0x08);
	pub const TAB: VirtualKey = VirtualKey(0x09);
	pub const CLEAR: VirtualKey = VirtualKey(0x0c);
	pub const RETURN: VirtualKey = VirtualKey(0x0d);

	pub const SHIFT: VirtualKey = VirtualKey(0x10);
	pub const CTRL: VirtualKey = VirtualKey(0x11);
	pub const ALT: VirtualKey = VirtualKey(0x12);
	pub const PAUSE: VirtualKey = VirtualKey(0x13);
	pub const CAPS_LOCK: VirtualKey = VirtualKey(0x14);
	pub const ESCAPE: VirtualKey = VirtualKey(0x1b);

	pub const SPACE: VirtualKey = VirtualKey(0x20);
	pub const PAGE_UP: VirtualKey = VirtualKey(0x21);
	pub const PAGE_DOWN: VirtualKey = VirtualKey(0x22);
	pub const END: VirtualKey = VirtualKey(0x23);
	pub const HOME: VirtualKey = VirtualKey(0x24);
	pub const LEFT: VirtualKey = VirtualKey(0x25);
	pub const UP: VirtualKey = VirtualKey(0x26);
	pub const RIGHT: VirtualKey = VirtualKey(0x27);
	pub const DOWN: VirtualKey = VirtualKey(0x28);
	pub const PRINT_SCREEN: VirtualKey = VirtualKey(0x2c);
	pub const INSERT: VirtualKey = VirtualKey(0x2d);
	pub const DELETE: VirtualKey = VirtualKey(0x2e);

	pub const NUMPAD0: VirtualKey = VirtualKey(0x60);
	pub const NUMPAD1: VirtualKey = VirtualKey(0x61);
	pub const NUMPAD2: VirtualKey = VirtualKey(0x62);
	pub const NUMPAD3: VirtualKey = VirtualKey(0x63);
	pub const NUMPAD4: VirtualKey = VirtualKey(0x64);
	pub const NUMPAD5: VirtualKey = VirtualKey(0x65);
	pub const NUMPAD6: VirtualKey = VirtualKey(0x66);
	pub const NUMPAD7: VirtualKey = VirtualKey(0x67);
	pub const NUMPAD8: VirtualKey = VirtualKey(0x68);
	pub const NUMPAD9: VirtualKey = VirtualKey(0x69);
	pub const MULTIPLY: VirtualKey = VirtualKey(0x6a);
	pub const ADD: VirtualKey = VirtualKey(0x6b);
	pub const ENTER: VirtualKey = VirtualKey(0x6c);
	pub const SUBTRACT: VirtualKey = VirtualKey(0x6d);
	pub const DECIMAL: VirtualKey = VirtualKey(0x6e);
	pub const DIVIDE: VirtualKey = VirtualKey(0x6f);

	pub const F1: VirtualKey = VirtualKey(0x70);
	pub const F2: VirtualKey = VirtualKey(0x71);
	pub const F3: VirtualKey = VirtualKey(0x72);
	pub const F4: VirtualKey = VirtualKey(0x73);
	pub const F5: VirtualKey = VirtualKey(0x74);
	pub const F6: VirtualKey = VirtualKey(0x75);
	pub const F7: VirtualKey = VirtualKey(0x76);
	pub const F8: VirtualKey = VirtualKey(0x77);
	pub const F9: VirtualKey = VirtualKey(0x78);
	pub const F10: VirtualKey = VirtualKey(0x79);
	pub const F11: VirtualKey = VirtualKey(0x7a);
	pub const F12: VirtualKey = VirtualKey(0x7b);

	pub const NUM_LOCK: VirtualKey = VirtualKey(0x90);
	pub const SCROLL_LOCK: VirtualKey = VirtualKey(0x91);

	pub const LSHIFT: VirtualKey = VirtualKey(0xa0);
	pub const RSHIFT: VirtualKey = VirtualKey(0xa1);
	pub const LCTRL: VirtualKey = VirtualKey(0xa2);
	pub const RCTRL: VirtualKey = VirtualKey(0xa3);
	pub const LALT: VirtualKey = VirtualKey(0xa4);
	pub const RALT: VirtualKey = VirtualKey(0xa5);
}
impl VirtualKey {
	/// Press a virtual key.
	#[inline]
	pub fn down(self) {
		unsafe { keybd_event(self.0, self.to_scan_code(), 0, 0); }
	}
	/// Release a virtual key.
	#[inline]
	pub fn up(self) {
		unsafe { keybd_event(self.0, self.to_scan_code(), KEYEVENTF_KEYUP, 0); }
	}
	/// Gets the async key state.
	#[inline]
	pub fn async_state(self) -> bool {
		unsafe { GetAsyncKeyState(self.0 as i32) as u16 & 0x8000 != 0 }
	}
	/// Translates the virtual key to a character, if possible.
	#[inline]
	pub fn to_char(self) -> Option<char> {
		unsafe {
			let value = MapVirtualKeyW(self.0 as u32, MAPVK_VK_TO_CHAR);
			if value == 0 {
				return None;
			}
			Some(std::char::from_u32_unchecked(value & 0x7ffffff))
		}
	}
	/// Translates the virtual key to a virtual scan code, if possible.
	#[inline]
	pub fn to_scan_code(self) -> u8 {
		unsafe { MapVirtualKeyW(self.0 as u32, MAPVK_VK_TO_VSC) as u8 }
	}
	/// Gets the virtual key for a virtual scan code.
	#[inline]
	pub fn from_scan_code(scan_code: u8) -> VirtualKey {
		unsafe { VirtualKey(MapVirtualKeyW(scan_code as u32, MAPVK_VSC_TO_VK) as u8) }
	}
	/// Gets the virtual key for a virtual scan code which distinguishes between left- and right-hand keys.
	#[inline]
	pub fn from_scan_code_ex(scan_code: u8) -> VirtualKey {
		unsafe { VirtualKey(MapVirtualKeyW(scan_code as u32, MAPVK_VSC_TO_VK_EX) as u8) }
	}
}
impl VirtualKey {
	/// Gets the name of a virtual key if there is one.
	#[inline]
	pub fn to_str(self) -> Option<&'static str> {
		Some(match self {
			VirtualKey::NONE => "NONE",
			VirtualKey::LBUTTON => "LBUTTON",
			VirtualKey::RBUTTON => "RBUTTON",
			VirtualKey::CANCEL => "CANCEL",
			VirtualKey::MBUTTON => "MBUTTON",
			VirtualKey::XBUTTON1 => "XBUTTON1",
			VirtualKey::XBUTTON2 => "XBUTTON2",
			VirtualKey::BACK => "BACK",
			VirtualKey::TAB => "TAB",
			VirtualKey::CLEAR => "CLEAR",
			VirtualKey::RETURN => "RETURN",

			VirtualKey::SHIFT => "SHIFT",
			VirtualKey::CTRL => "CTRL",
			VirtualKey::ALT => "ALT",
			VirtualKey::PAUSE => "PAUSE",
			VirtualKey::CAPS_LOCK => "CAPS_LOCK",
			VirtualKey::ESCAPE => "ESCAPE",

			VirtualKey::SPACE => "SPACE",
			VirtualKey::PAGE_UP => "PAGE_UP",
			VirtualKey::PAGE_DOWN => "PAGE_DOWN",
			VirtualKey::END => "END",
			VirtualKey::HOME => "HOME",
			VirtualKey::LEFT => "LEFT",
			VirtualKey::UP => "UP",
			VirtualKey::RIGHT => "RIGHT",
			VirtualKey::DOWN => "DOWN",
			VirtualKey::PRINT_SCREEN => "PRINT_SCREEN",
			VirtualKey::INSERT => "INSERT",
			VirtualKey::DELETE => "DELETE",

			VirtualKey(b'0') => "0",
			VirtualKey(b'1') => "1",
			VirtualKey(b'2') => "2",
			VirtualKey(b'3') => "3",
			VirtualKey(b'4') => "4",
			VirtualKey(b'5') => "5",
			VirtualKey(b'6') => "6",
			VirtualKey(b'7') => "7",
			VirtualKey(b'8') => "8",
			VirtualKey(b'9') => "9",

			VirtualKey(b'A') => "A",
			VirtualKey(b'B') => "B",
			VirtualKey(b'C') => "C",
			VirtualKey(b'D') => "D",
			VirtualKey(b'E') => "E",
			VirtualKey(b'F') => "F",
			VirtualKey(b'G') => "G",
			VirtualKey(b'H') => "H",
			VirtualKey(b'I') => "I",
			VirtualKey(b'J') => "J",
			VirtualKey(b'K') => "K",
			VirtualKey(b'L') => "L",
			VirtualKey(b'M') => "M",
			VirtualKey(b'N') => "N",
			VirtualKey(b'O') => "O",

			VirtualKey(b'P') => "P",
			VirtualKey(b'Q') => "Q",
			VirtualKey(b'R') => "R",
			VirtualKey(b'S') => "S",
			VirtualKey(b'T') => "T",
			VirtualKey(b'U') => "U",
			VirtualKey(b'V') => "V",
			VirtualKey(b'W') => "W",
			VirtualKey(b'X') => "X",
			VirtualKey(b'Y') => "Y",
			VirtualKey(b'Z') => "Z",

			VirtualKey::NUMPAD0 => "NUMPAD0",
			VirtualKey::NUMPAD1 => "NUMPAD1",
			VirtualKey::NUMPAD2 => "NUMPAD2",
			VirtualKey::NUMPAD3 => "NUMPAD3",
			VirtualKey::NUMPAD4 => "NUMPAD4",
			VirtualKey::NUMPAD5 => "NUMPAD5",
			VirtualKey::NUMPAD6 => "NUMPAD6",
			VirtualKey::NUMPAD7 => "NUMPAD7",
			VirtualKey::NUMPAD8 => "NUMPAD8",
			VirtualKey::NUMPAD9 => "NUMPAD9",
			VirtualKey::MULTIPLY => "MULTIPLY",
			VirtualKey::ADD => "ADD",
			VirtualKey::ENTER => "ENTER",
			VirtualKey::SUBTRACT => "SUBTRACT",
			VirtualKey::DECIMAL => "DECIMAL",
			VirtualKey::DIVIDE => "DIVIDE",

			VirtualKey::F1 => "F1",
			VirtualKey::F2 => "F2",
			VirtualKey::F3 => "F3",
			VirtualKey::F4 => "F4",
			VirtualKey::F5 => "F5",
			VirtualKey::F6 => "F6",
			VirtualKey::F7 => "F7",
			VirtualKey::F8 => "F8",
			VirtualKey::F9 => "F9",
			VirtualKey::F10 => "F10",
			VirtualKey::F11 => "F11",
			VirtualKey::F12 => "F12",

			VirtualKey::NUM_LOCK => "NUM_LOCK",
			VirtualKey::SCROLL_LOCK => "SCROLL_LOCK",

			VirtualKey::LSHIFT => "LSHIFT",
			VirtualKey::RSHIFT => "RSHIFT",
			VirtualKey::LCTRL => "LCTRL",
			VirtualKey::RCTRL => "RCTRL",
			VirtualKey::LALT => "LALT",
			VirtualKey::RALT => "RALT",

			_ => return None,
		})
	}
}
impl std::fmt::Display for VirtualKey {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self.to_str() {
			Some(s) => s.fmt(f),
			None => self.0.fmt(f),
		}
	}
}

/// Error returned when the string does not match a known virtual key name.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct VirtualKeyFromStrError {}
impl std::fmt::Display for VirtualKeyFromStrError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		"unknown virtual key name".fmt(f)
	}
}
impl std::error::Error for VirtualKeyFromStrError {
	fn description(&self) -> &str {
		"unknown virtual key name"
	}
}

impl std::str::FromStr for VirtualKey {
	type Err = VirtualKeyFromStrError;
	fn from_str(s: &str) -> Result<VirtualKey, VirtualKeyFromStrError> {
		for i in 0..255 {
			let vk = VirtualKey(i);
			if let Some(vk_str) = vk.to_str() {
				if stricmp(s, vk_str) {
					return Ok(vk);
				}
			}
		}
		Err(VirtualKeyFromStrError {})
	}
}
fn stricmp(a: &str, b: &str) -> bool {
	if a.len() != b.len() {
		return false;
	}
	return a.bytes()
		.map(|chr| chr.to_ascii_uppercase())
		.eq(b.bytes());
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

#[test]
fn test_vk_str() {
	assert_eq!("xbutton1".parse(), Ok(VirtualKey::XBUTTON1));
	assert_eq!("lalt".parse(), Ok(VirtualKey::LALT));
}

#[test]
fn test_vk_scan_codes() {
	for scan_code in 0..256 {
		let vk = VirtualKey::from_scan_code(scan_code as u8);
		println!("{:#x} {:?} = {}", vk.0, vk, vk);
	}
}

#[test]
#[ignore]
fn print_table() {
	let mut string = String::new();
	let mut indices = [0u16; 256];
	let mut start = 0;
	for i in 0..256 {
		let vk = VirtualKey(i as u8);
		if let Some(vk_str) = vk.to_str() {
			string.push_str(vk_str);
			start = string.len() as u16;
		}
		indices[i] = start;
	}
	println!("let string = {:?};", string);
	print!("let indices = &[0");
	for idx in &indices[..] {
		print!(",{}", idx);
	}
	println!("];");
}
