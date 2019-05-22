use external::wndclass::{pump_once, sleep};
use external::windows_hook;
use external::mouse::MouseInput;

use std::sync::atomic;

static MOUSE_X: atomic::AtomicI32 = atomic::AtomicI32::new(0);
static MOUSE_Y: atomic::AtomicI32 = atomic::AtomicI32::new(0);

unsafe fn mouse_move(context: &mut external::hook::MouseLL) {
	match context.mouse_data() {
		external::hook::MouseData::Move => {
			if context.injected() || context.lower_il_injected() {
				context.clear_injected();
				context.clear_lower_il_injected();
				MOUSE_X.store(context.pt_x(), atomic::Ordering::SeqCst);
				MOUSE_Y.store(context.pt_y(), atomic::Ordering::SeqCst);
			}
			else {
				let oldx = MOUSE_X.swap(context.pt_x(), atomic::Ordering::SeqCst);
				let oldy = MOUSE_Y.swap(context.pt_y(), atomic::Ordering::SeqCst);
				if oldx != 0 && oldy != 0 {
					let dx = context.pt_x() - oldx;
					let dy = context.pt_y() - oldy;
					MouseInput::mouse_move(dx, dy).send();
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
		sleep(0);
	}
}
