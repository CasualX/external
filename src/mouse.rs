/*!
Mouse input.
!*/

use crate::winapi::*;

#[derive(Copy, Clone, Debug)]
pub struct Mouse;
impl Mouse {
	/// Move the mouse relatively.
	#[inline]
	pub fn mouse_move(self, dx: i32, dy: i32) {
		unsafe { mouse_event(MOUSEEVENTF_MOVE, dx as DWORD, dy as DWORD, 0, 0); }
	}
	/// Set the mouse position in absolute pixel coordinates.
	#[inline]
	pub fn mouse_set(self, dx: u32, dy: u32) {
		unsafe { mouse_event(MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE, dx as DWORD, dy as DWORD, 0, 0); }
	}
	/// Scroll the mouse wheel.
	#[inline]
	pub fn mouse_wheel(self, delta: i32) {
		unsafe { mouse_event(MOUSEEVENTF_WHEEL, 0, 0, delta as DWORD, 0); }
	}

	/// Interact with the left mouse button.
	#[inline]
	pub fn left(self, down: bool) {
		if down { self.left_down() }
		else { self.left_up() }
	}
	/// Press the left mouse button.
	#[inline]
	pub fn left_down(self) {
		unsafe { mouse_event(MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0); }
	}
	/// Release the left mouse button.
	#[inline]
	pub fn left_up(self) {
		unsafe { mouse_event(MOUSEEVENTF_LEFTUP, 0, 0, 0, 0); }
	}

	/// Interact with the right mouse button.
	#[inline]
	pub fn right(self, down: bool) {
		if down { self.right_down() }
		else { self.right_up() }
	}
	/// Press the right mouse button.
	#[inline]
	pub fn right_down(self) {
		unsafe { mouse_event(MOUSEEVENTF_RIGHTDOWN, 0, 0, 0, 0); }
	}
	/// Release the right mouse button.
	#[inline]
	pub fn right_up(self) {
		unsafe { mouse_event(MOUSEEVENTF_RIGHTUP, 0, 0, 0, 0); }
	}

	/// Interact with the middle mouse button.
	#[inline]
	pub fn middle(self, down: bool) {
		if down { self.middle_down() }
		else { self.middle_up() }
	}
	/// Press the middle mouse button.
	#[inline]
	pub fn middle_down(self) {
		unsafe { mouse_event(MOUSEEVENTF_MIDDLEDOWN, 0, 0, 0, 0); }
	}
	/// Release the middle mouse button.
	#[inline]
	pub fn middle_up(self) {
		unsafe { mouse_event(MOUSEEVENTF_MIDDLEUP, 0, 0, 0, 0); }
	}

	/// Interact with the xbutton1 mouse button.
	#[inline]
	pub fn xbutton1(self, down: bool) {
		if down { self.xbutton1_down() }
		else { self.xbutton1_up() }
	}
	/// Press the xbutton1 mouse button.
	#[inline]
	pub fn xbutton1_down(self) {
		unsafe { mouse_event(MOUSEEVENTF_XDOWN, 0, 0, XBUTTON1 as DWORD, 0); }
	}
	/// Release the xbutton1 mouse button.
	#[inline]
	pub fn xbutton1_up(self) {
		unsafe { mouse_event(MOUSEEVENTF_XUP, 0, 0, XBUTTON1 as DWORD, 0); }
	}

	/// Interact with the xbutton2 mouse button.
	#[inline]
	pub fn xbutton2(self, down: bool) {
		if down { self.xbutton2_down() }
		else { self.xbutton2_up() }
	}
	/// Press the xbutton2 mouse button.
	#[inline]
	pub fn xbutton2_down(self) {
		unsafe { mouse_event(MOUSEEVENTF_XDOWN, 0, 0, XBUTTON2 as DWORD, 0); }
	}
	/// Release the xbutton2 mouse button.
	#[inline]
	pub fn xbutton2_up(self) {
		unsafe { mouse_event(MOUSEEVENTF_XUP, 0, 0, XBUTTON2 as DWORD, 0); }
	}

	/// Gets the primary screen size for use with mouse movement.
	#[inline]
	pub fn primary_screen_size(self) -> (u32, u32) {
		unsafe {
			let width = GetSystemMetrics(SM_CXSCREEN);
			let height = GetSystemMetrics(SM_CYSCREEN);
			(width as u32, height as u32)
		}
	}
}

#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct MouseInput {
	dx: i32,
	dy: i32,
	mouse_data: u32,
	flags: u32,
}
impl MouseInput {
	#[must_use]
	pub const fn mouse_move(dx: i32, dy: i32) -> MouseInput {
		MouseInput { dx, dy, mouse_data: 0, flags: MOUSEEVENTF_MOVE }
	}
	#[must_use]
	pub const fn mouse_absmove(x: u32, y: u32) -> MouseInput {
		MouseInput { dx: x as i32, dy: y as i32, mouse_data: 0, flags: MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE }
	}
	pub fn send(self) {
		unsafe { mouse_event(self.flags, self.dx as u32, self.dy as u32, self.mouse_data, 0); }
	}
}

// MouseInput::mouse_move(1, 1).send();
