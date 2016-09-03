/*!
Tries to hide injected keypresses.
*/

#[macro_use]
extern crate external;

use ::std::{thread, time};

use ::external::wndclass::{pump_once};

windows_hook! {
	fn hide_keyboard_inject(context: &mut KeyboardLL) {
		context.clear_injected();
		context.clear_lower_il_injected();
	}
}
windows_hook! {
	fn hide_mouse_inject(context: &mut MouseLL) {
		context.clear_injected();
		context.clear_lower_il_injected();
	}
}

fn main() {
	println!("Scrub the injected flag from low level key and mouse events.
	          Run this in the background after `cargo run --example input_detect` and see.");
	
	let _keybd_hook = hide_keyboard_inject();
	let _mouse_hook = hide_mouse_inject();
	while pump_once() {
		thread::sleep(time::Duration::from_millis(1));
	}
}
