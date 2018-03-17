/*!
*/

use std::{ptr, mem, panic};

use winapi::um::winuser::{RegisterClassExW, CreateWindowExW, DefWindowProcW};
use winapi::um::winuser::{GetMessageW, PeekMessageW, TranslateMessage, DispatchMessageW};
use winapi::um::winuser::{WNDCLASSEXW, MSG};
use winapi::um::winuser::{COLOR_WINDOWFRAME, WS_OVERLAPPEDWINDOW, CW_USEDEFAULT, CS_VREDRAW, CS_HREDRAW, WM_QUIT, WM_NCCREATE, PM_REMOVE};
use winapi::um::synchapi::{Sleep};
use winapi::shared::ntdef::{LPCWSTR};
use winapi::shared::windef::{HWND, HICON, HCURSOR, HBRUSH};
use winapi::shared::minwindef::{UINT, WPARAM, LPARAM, HINSTANCE, LRESULT, TRUE};

use window::Window;
use error::ErrorCode;
use {Result, FromInner, IntoInner};

pub static CLASS_NAME: [u16; 6] = wide_str!('C' 'l' 'a' 's' 's' 0);
pub static WINDOW_TITLE: [u16; 6] = wide_str!('T' 'i' 't' 'l' 'e' 0);

extern "C" {
	static __ImageBase: u8;
}

#[allow(non_snake_case)]
pub struct Message {
	pub window: Window,
	pub message: UINT,
	pub wParam: WPARAM,
	pub lParam: LPARAM,
	pub result: LRESULT,
}

pub trait WndClass {
	fn class() -> WNDCLASSEXW {
		WNDCLASSEXW {
			cbSize: mem::size_of::<WNDCLASSEXW>() as u32,
			style: CS_VREDRAW | CS_HREDRAW,
			lpfnWndProc: Some(Self::thunk_wnd_proc),
			cbClsExtra: 0,
			cbWndExtra: 0,
			hInstance: unsafe { &__ImageBase as *const _ as HINSTANCE },
			hIcon: 0 as HICON,
			hCursor: 0 as HCURSOR,
			hbrBackground: (COLOR_WINDOWFRAME) as HBRUSH,
			lpszMenuName: 0 as LPCWSTR,
			lpszClassName: &CLASS_NAME as *const u16,
			hIconSm: 0 as HICON,
		}
	}
	fn register() -> Result<()> {
		unsafe {
			let class = Self::class();
			if RegisterClassExW(&class) == 0 {
				Err(ErrorCode::last())
			}
			else {
				Ok(())
			}
		}
	}

	fn create() -> Result<Window> {
		unsafe {
			let class = Self::class();
			let hwnd = CreateWindowExW(
				0,
				class.lpszClassName,
				&WINDOW_TITLE as *const u16,
				WS_OVERLAPPEDWINDOW,
				CW_USEDEFAULT, CW_USEDEFAULT, 800, 600,
				ptr::null_mut(),
				ptr::null_mut(),
				class.hInstance,
				ptr::null_mut()
			);
			if hwnd.is_null() {
				Err(ErrorCode::last())
			}
			else {
				Ok(Window::from_inner(hwnd))
			}
		}
	}

	fn wnd_proc(msg: &mut Message);
	fn def_wnd_proc(msg: &mut Message) {
		unsafe {
			if msg.message == WM_NCCREATE {

			}
			msg.result = DefWindowProcW(msg.window.into_inner(), msg.message, msg.wParam, msg.lParam);
		}
	}

	#[allow(non_snake_case)]
	#[doc(hidden)]
	unsafe extern "system" fn thunk_wnd_proc(hwnd: HWND, msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT {
		let result = panic::catch_unwind(|| {
			let mut msg = Message {
				window: Window::from_inner(hwnd),
				message: msg,
				wParam: wParam,
				lParam: lParam,
				result: 0,
			};
			Self::wnd_proc(&mut msg);
			msg
		});
		result.unwrap().result
	}
}

pub fn pump_once() -> bool {
	unsafe {
		let mut msg: MSG = mem::zeroed();
		while PeekMessageW(&mut msg, 0 as HWND, 0, 0, PM_REMOVE) == TRUE {
			TranslateMessage(&mut msg);
			DispatchMessageW(&mut msg);
		}
		msg.message != WM_QUIT
	}
}

pub fn pump_thread() {
	unsafe {
		let mut msg: MSG = mem::zeroed();
		while GetMessageW(&mut msg, 0 as HWND, 0, 0) > 0 {
			TranslateMessage(&mut msg);
			DispatchMessageW(&mut msg);
		}
	}
}

pub fn sleep(ms: u32) {
	unsafe {
		Sleep(ms);
	}
}
