/*!
Tries to detect inject keypresses.
*/

#[macro_use]
extern crate external;

use ::std::{thread, time};

use ::external::wndclass::{pump_once};

windows_hook! {
	fn detect_keybd_input(context: &mut KeyboardLL) {
		if context.injected() || context.lower_il_injected() {
			println!("Injected keybd_event detected: {:#?}", context);
		}
	}
}

windows_hook! {
	fn detect_mouse_input(context: &mut MouseLL) {
		if context.injected() || context.lower_il_injected() {
			println!("Injected mouse_event detected: {:#?}", context);
		}
	}
}

fn main() {
	println!("Detect injected key and mouse events.
	          Now run `cargo run --example input_inject` concurrently and see.");

	// Install the low level keyboard hook
	let _keybd_hook = detect_keybd_input();
	let _mouse_hook = detect_mouse_input();
	// Exit only available with ctrl-C
	// Make sure we can receive messages
	while pump_once() {
		// Don't hog the CPU
		thread::sleep(time::Duration::from_millis(1));
	}
}
