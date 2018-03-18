#[macro_use]
extern crate external;

use external::wndclass::{pump_once};

use std::time::{Duration};
use std::thread::{sleep};
use std::sync::atomic;

static MOUSE_X: atomic::AtomicIsize = atomic::ATOMIC_ISIZE_INIT;
static MOUSE_Y: atomic::AtomicIsize = atomic::ATOMIC_ISIZE_INIT;

unsafe fn mouse_move(context: &mut external::hook::MouseLL) {
	match context.mouse_data() {
		external::hook::MouseData::Move => {
			if context.injected() || context.lower_il_injected() {
				context.clear_injected();
				context.clear_lower_il_injected();
				MOUSE_X.store(context.pt_x() as isize, atomic::Ordering::SeqCst);
				MOUSE_Y.store(context.pt_y() as isize, atomic::Ordering::SeqCst);
			}
			else {
				let oldx = MOUSE_X.swap(context.pt_x() as isize, atomic::Ordering::SeqCst);
				let oldy = MOUSE_Y.swap(context.pt_y() as isize, atomic::Ordering::SeqCst);
				if oldx != 0 && oldy != 0 {
					let dx = context.pt_x() as isize - oldx;
					let dy = context.pt_y() as isize - oldy;
					external::input::mouse_move(dx as i32, dy as i32);
					print!("\rdx:{} dy:{}                  ", dx, dy);
					context.cancel();
				}
			}
		},
		_ => (),
	}
}

windows_hook! {
	fn analytics(context: &mut MouseLL) {
		unsafe {
			mouse_move(context);
		}
	}
}

fn main() {
	println!("Mouse mirror.");
	let _hook = analytics().unwrap();

	while pump_once() {
		sleep(Duration::new(0, 0));
	}
}
