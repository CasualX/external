use winapi::um::profileapi::{QueryPerformanceCounter, QueryPerformanceFrequency};

static mut TIME_BASE: u64 = 0;

/// Returns the time in seconds since the first time this function was called.
pub fn time_s() -> f64 {
	unsafe {
		let mut counter = 0u64;
		let mut frequency = 0u64;
		QueryPerformanceCounter(&mut counter as *mut _ as *mut _);
		QueryPerformanceFrequency(&mut frequency as *mut _ as *mut _);
		if TIME_BASE == 0 {
			TIME_BASE = counter;
			0.0
		}
		else {
			(counter - TIME_BASE) as f64 / frequency as f64
		}
	}
}
