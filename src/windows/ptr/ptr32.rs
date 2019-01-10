
use std::{cmp, fmt, ops, mem};
use std::marker::PhantomData;

use super::Pod;

/// 32bit Typed Pointer.
#[repr(C)]
pub struct Ptr32<T: ?Sized>(u32, PhantomData<fn() -> T>);

impl<T: ?Sized> From<u32> for Ptr32<T> {
	fn from(address: u32) -> Ptr32<T> {
		Ptr32(address, PhantomData)
	}
}
impl<T: ?Sized> From<Ptr32<T>> for u32 {
	fn from(ptr: Ptr32<T>) -> u32 {
		ptr.0
	}
}

impl<T: ?Sized> Ptr32<T> {
	/// Returns a raw null pointer.
	pub fn null() -> Ptr32<T> {
		Ptr32(0, PhantomData)
	}
	/// Returns if the pointer is the null pointer.
	pub fn is_null(self) -> bool {
		self.0 == 0
	}
	/// Converts to a raw integer value.
	pub fn into_raw(self) -> u32 {
		self.0
	}
}
impl<T> Ptr32<[T]> {
	pub fn decay(self) -> Ptr32<T> {
		Ptr32(self.0, PhantomData)
	}
	pub fn at(self, index: usize) -> Ptr32<T> {
		Ptr32(self.0 + mem::size_of::<T>() as u32 * index as u32, PhantomData)
	}
}
impl<T> ops::Sub for Ptr32<T> {
	type Output = i32;
	fn sub(self, rhs: Ptr32<T>) -> i32 {
		(u32::wrapping_sub(self.0, rhs.0) as i32) / mem::size_of::<T>() as i32
	}
}
impl<T: ?Sized> fmt::Display for Ptr32<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:>#010X}", self.0)
	}
}
impl<T: ?Sized> fmt::Debug for Ptr32<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Ptr32({:>#010X})", self.0)
	}
}
impl<T> ops::Add<u32> for Ptr32<T> {
	type Output = Ptr32<T>;
	fn add(self, rhs: u32) -> Ptr32<T> {
		Ptr32(self.0 + rhs * mem::size_of::<T>() as u32, self.1)
	}
}
impl<T> ops::Sub<u32> for Ptr32<T> {
	type Output = Ptr32<T>;
	fn sub(self, rhs: u32) -> Ptr32<T> {
		Ptr32(self.0 - rhs * mem::size_of::<T>() as u32, self.1)
	}
}
impl<T: ?Sized> Clone for Ptr32<T> {
	fn clone(&self) -> Self {
		Ptr32(self.0, self.1)
	}
}
impl<T: ?Sized> Default for Ptr32<T> {
	fn default() -> Ptr32<T> {
		Ptr32::null()
	}
}
impl<T: ?Sized> PartialEq for Ptr32<T> {
	fn eq(&self, rhs: &Self) -> bool {
		self.0 == rhs.0
	}
}
impl<T: ?Sized> PartialOrd for Ptr32<T> {
	fn partial_cmp(&self, rhs: &Ptr32<T>) -> Option<cmp::Ordering> {
		self.0.partial_cmp(&rhs.0)
	}
}
impl<T: ?Sized> Copy for Ptr32<T> {}
impl<T: ?Sized> Eq for Ptr32<T> {}
impl<T: ?Sized> Ord for Ptr32<T> {
	fn cmp(&self, rhs: &Ptr32<T>) -> cmp::Ordering {
		self.0.cmp(&rhs.0)
	}
}

unsafe impl<T: ?Sized> Pod for Ptr32<T> {}

//----------------------------------------------------------------

#[cfg(test)]
mod tests {
	use std::mem;
	use super::*;

	#[test]
	fn ptr32() {
		let a = Ptr32::<f32>::from(0x2000);
		let b = a + 0x40;
		let c = a - 0x40;
		assert_eq!(mem::size_of_val(&a), 4);
		assert_eq!(c - a, -0x40);
		assert_eq!(b.into_raw(), 0x2100);
		assert_eq!(format!("{}", a), "0x00002000");
		assert_eq!(c.into_raw(), 0x1F00);
	}
}
