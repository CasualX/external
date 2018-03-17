/*!
Track the total mouse distance moved.
*/

#[macro_use]
extern crate external;

use external::wndclass::{pump_once};

use std::time::{self, Duration, SystemTime, UNIX_EPOCH};

// Uninitialized mouse coordinate value.
const PT_UNINIT: i32 = 0x80000000u32 as i32;

static mut MOUSE_PT_X: i32 = PT_UNINIT;
static mut MOUSE_PT_Y: i32 = PT_UNINIT;

static mut MOUSE_DX: i32 = 0;
static mut MOUSE_DY: i32 = 0;

static mut MOUSE_TIME: f64 = 0.0;
static mut MOUSE_DT: f64 = 0.0;

windows_hook! {
	fn mouse_stats(context: &mut MouseLL) {
		unsafe {
			if MOUSE_PT_X != PT_UNINIT {
				MOUSE_DX += (context.pt_x() - MOUSE_PT_X).abs();
			}
			if MOUSE_PT_Y != PT_UNINIT {
				MOUSE_DY += (context.pt_y() - MOUSE_PT_Y).abs();
			}
			MOUSE_PT_X = context.pt_x();
			MOUSE_PT_Y = context.pt_y();

			let time = time::precise_time_s();
			let dt = time - MOUSE_TIME;
			MOUSE_TIME = time;
			MOUSE_DT = dt;
		}
	}
}

fn main() {
	println!("Track the total mouse distance moved.");
	let _hook = mouse_stats().unwrap();
	std::thread::spawn(|| {
		let mut dx = 1;
		loop {
			dx = -dx;
			external::input::mouse_move(dx, 0);
			::std::thread::sleep(Duration::new(0, 1000));
		}
	});
	while pump_once() {
		unsafe {
			print!("\rdx:{} dy:{} dt:{}        ", MOUSE_DX, MOUSE_DY, MOUSE_DT);
		}
	}
}
