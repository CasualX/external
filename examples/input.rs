/*!
Playground for detecting and hiding injected input.
!*/

use std::{env, thread, time};

use external::vk::VirtualKey;
use external::wndclass::{pump_once, sleep};
use external::windows_hook;
use external::hook::{KeyboardLL, MouseLL};

fn main() {
	let mut args = env::args();

	if let (Some(_), Some(cmd), None) = (args.next(), args.next(), args.next()) {
		match &*cmd {
			"detect" => detect(),
			"hide" => hide(),
			"inject" => inject(),
			_ => print!("Unrecognized command: {}\nTry one of: detect, hide, inject\n", cmd),
		}
	}
	else {
		println!("Playground for detecting and hiding injected input.");
		println!("Give any of: detect, hide or inject arguments.");
	}
}

//----------------------------------------------------------------
// Detect injected input

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

fn detect() {
	println!("Detect injected key and mouse events.\nNow run `cargo run --example input -- inject` concurrently and observe.");

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

//----------------------------------------------------------------
// Hide injected input

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

fn hide() {
	println!("Scrub the injected flag from low level key and mouse events.\nRun this in the background after `cargo run --example input -- detect` and observe.");
	
	let _keybd_hook = hide_keyboard_inject();
	let _mouse_hook = hide_mouse_inject();
	while pump_once() {
		thread::sleep(time::Duration::from_millis(1));
	}
}

//----------------------------------------------------------------
// Inject space input

fn inject() {
	VirtualKey::SPACE.down();
	sleep(100);
	VirtualKey::SPACE.up();
}
