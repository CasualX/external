/*!
Example demonstrating the usage of low level hooks.
*/

#![allow(unused_variables)]

#[macro_use]
extern crate external;

use external::input::vk;
use external::wndclass::{pump_once, sleep};

// Any communcation has to happen through global mutable state...
static mut DONE: bool = false;

windows_hook! {
	pub fn keybd_hook(context: &mut KeyboardLL) {
		if context.vk_code() == vk::RETURN {
			unsafe { DONE = true; }
		}
		else {
			println!("{:#?}", context);
		}
	}
}

windows_hook! {
	pub fn mouse_hook(context: &mut MouseLL) {
		println!("{:?}", context.mouse_data());
	}
}

fn main() {
	// Instantiate the hooks.
	let _hk = keybd_hook().unwrap();
	let _hm = mouse_hook().unwrap();

	// If the hook was not instantiated on a GUI thread it is required to pump messages or you will not receive callbacks.
	while unsafe { !DONE } && pump_once() {
		// Consider not hogging all the CPU time
		sleep(1);
	}

	// The hooks are unhooked when they go out of scope, here done explicitly.
	drop(_hk);
	drop(_hm);
}
