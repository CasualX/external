
/// Assert a type is plain old data.
///
/// This is used to verify that such types are safe to be transferred between processes.
pub unsafe trait Pod {}

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

//unsafe impl Pod for .. {}
