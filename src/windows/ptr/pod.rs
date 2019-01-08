
use std::{mem, slice};

/// Assert a type is plain old data.
///
/// This is used to verify that such types are safe to be transferred between processes.
pub unsafe trait Pod {
	fn as_bytes(&self) -> &[u8] {
		unsafe { slice::from_raw_parts(self as *const _ as *const _, mem::size_of_val(self)) }
	}
	fn as_bytes_mut(&mut self) -> &mut [u8] {
		unsafe { slice::from_raw_parts_mut(self as *mut _ as *mut _, mem::size_of_val(self)) }
	}
}

// unsafe impl Pod for .. {}

unsafe impl Pod for u8 {}
unsafe impl Pod for u16 {}
unsafe impl Pod for u32 {}
unsafe impl Pod for u64 {}

unsafe impl Pod for i8 {}
unsafe impl Pod for i16 {}
unsafe impl Pod for i32 {}
unsafe impl Pod for i64 {}

unsafe impl Pod for f32 {}
unsafe impl Pod for f64 {}

unsafe impl<T: Pod> Pod for [T] {}

macro_rules! impl_pod_array {
	($n:tt $($tail:tt)+) => {
		unsafe impl<T: Pod> Pod for [T; $n] {}
		impl_pod_array!($($tail)+);
	};
	($n:tt) => {
		unsafe impl<T: Pod> Pod for [T; $n] {}
	};
}
impl_pod_array!(0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15);
impl_pod_array!(16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31);
impl_pod_array!(32 48 64 80 100 128 160 192 256 512 768 1024 2048 4096);
