/*

use super::{CallKeyboardLL, KeyboardLL, CallMouseLL, MouseLL};
use winapi::{c_int, WM_KEYDOWN, WM_SYSKEYDOWN, WM_KEYUP, WM_SYSKEYUP};
use winapi::{WM_LBUTTONDOWN, WM_LBUTTONUP, WM_RBUTTONDOWN, WM_RBUTTONUP, WM_MBUTTONDOWN, WM_MBUTTONUP};
use winapi::{VK_LBUTTON, VK_RBUTTON, VK_MBUTTON, VK_XBUTTON1, VK_XBUTTON2};

enum KeyLogger {}
impl_invoker!(KeyLogger);
impl CallKeyboardLL for KeyLogger {
	fn callback(context: &mut KeyboardLL) {
		match context.message() {
			WM_KEYDOWN => vk_set(context.vk_code()),
			WM_KEYUP => vk_clear(context.vk_code()),
			WM_SYSKEYDOWN => vk_set(context.vk_code()),
			WM_SYSKEYUP => vk_clear(context.vk_code()),
			_ => (),
		}
	}
}

enum MouseLogger {}
impl_invoker!(MouseLogger);
impl CallMouseLL for MouseLogger {
	fn callback(context: &mut MouseLL) {
		match context.message() {
			WM_LBUTTONDOWN => vk_set(VK_LBUTTON),
			WM_LBUTTONUP => vk_clear(VK_LBUTTON),
			WM_RBUTTONDOWN => vk_set(VK_RBUTTON),
			WM_RBUTTONUP => vk_clear(VK_RBUTTON),
			WM_MBUTTONDOWN => vk_set(VK_MBUTTON),
			WM_MBUTTONUP => vk_clear(VK_MBUTTON),
			_ => (),
		}
	}
}

static mut vk_state: [u32; 8] = [0u32; 8];

fn vk_set(vk: c_int) {
	let vk = vk as u32;
	if vk < 256 {
		unsafe {
			vk_state[(vk / 32) as usize] |= 1 << (vk % 32);
		}
	}
}
fn vk_clear(vk: c_int) {
	let vk = vk as u32;
	if vk < 256 {
		unsafe {
			vk_state[(vk / 32) as usize] &= !(1 << (vk % 32));
		}
	}
}
fn vk_test(vk: c_int) -> bool {
	let vk = vk as u32;
	if vk < 256 {
		unsafe {
			(vk_state[(vk / 32) as usize] & (1 << (vk % 32))) != 0
		}
	}
	else {
		false
	}
}
*/
fn main() {}
