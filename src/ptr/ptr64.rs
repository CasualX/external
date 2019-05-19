use std::{cmp, fmt, ops, mem};
use std::marker::PhantomData;
use super::Pod;

/// 64bit Typed Pointer.
#[repr(C)]
pub struct Ptr64<T: ?Sized = ()>(u64, PhantomData<fn() -> T>);

impl<T: ?Sized> From<u64> for Ptr64<T> {
	fn from(addr: u64) -> Ptr64<T> {
		Ptr64(addr, PhantomData)
	}
}
impl<T: ?Sized> From<Ptr64<T>> for u64 {
	fn from(ptr: Ptr64<T>) -> u64 {
		ptr.0
	}
}

impl<T: ?Sized> Ptr64<T> {
	/// Returns a raw null pointer.
	pub fn null() -> Ptr64<T> {
		Ptr64(0, PhantomData)
	}
	/// Returns if the pointer is the null pointer.
	pub fn is_null(self) -> bool {
		self.0 == 0
	}
	/// Constructs a pointer from base and offset.
	pub fn member(base: u64, offset: u32) -> Ptr64<T> {
		Ptr64(base + offset as u64, PhantomData)
	}
	/// Casts the pointer to a different type keeping the pointer address fixed.
	pub fn cast<U: ?Sized>(self) -> Ptr64<U> {
		Ptr64(self.0, PhantomData)
	}
	/// Offsets and casts the pointer.
	///
	/// Because the type of the current and the target may be unrelated, this is a byte offset.
	pub fn offset<U: ?Sized>(self, offset: i64) -> Ptr64<U> {
		let addr = self.0.wrapping_add(offset as u64);
		Ptr64(addr, PhantomData)
	}
	/// Converts to a raw integer value.
	pub fn into_raw(self) -> u64 {
		self.0
	}
}
impl<T> Ptr64<[T]> {
	pub fn decay(self) -> Ptr64<T> {
		Ptr64(self.0, PhantomData)
	}
	pub fn at(self, index: usize) -> Ptr64<T> {
		Ptr64(self.0 + mem::size_of::<T>() as u64 * index as u64, PhantomData)
	}
}
impl<T> ops::Sub for Ptr64<T> {
	type Output = i64;
	fn sub(self, rhs: Ptr64<T>) -> i64 {
		(u64::wrapping_sub(self.0, rhs.0) as i64) / mem::size_of::<T>() as i64
	}
}
impl<T: ?Sized> fmt::Display for Ptr64<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:>#018X}", self.0)
	}
}
impl<T: ?Sized> fmt::Debug for Ptr64<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Ptr64({:>#018X})", self.0)
	}
}
impl<T> ops::Add<u64> for Ptr64<T> {
	type Output = Ptr64<T>;
	fn add(self, rhs: u64) -> Ptr64<T> {
		Ptr64(self.0 + rhs * mem::size_of::<T>() as u64, self.1)
	}
}
impl<T> ops::Sub<u64> for Ptr64<T> {
	type Output = Ptr64<T>;
	fn sub(self, rhs: u64) -> Ptr64<T> {
		Ptr64(self.0 - rhs * mem::size_of::<T>() as u64, self.1)
	}
}
impl<T: ?Sized> Clone for Ptr64<T> {
	fn clone(&self) -> Self {
		Ptr64(self.0, self.1)
	}
}
impl<T: ?Sized> Default for Ptr64<T> {
	fn default() -> Ptr64<T> {
		Ptr64::null()
	}
}
impl<T: ?Sized> PartialEq for Ptr64<T> {
	fn eq(&self, rhs: &Self) -> bool {
		self.0 == rhs.0
	}
}
impl<T: ?Sized> PartialOrd for Ptr64<T> {
	fn partial_cmp(&self, rhs: &Ptr64<T>) -> Option<cmp::Ordering> {
		self.0.partial_cmp(&rhs.0)
	}
}
impl<T: ?Sized> Copy for Ptr64<T> {}
impl<T: ?Sized> Eq for Ptr64<T> {}
impl<T: ?Sized> Ord for Ptr64<T> {
	fn cmp(&self, rhs: &Ptr64<T>) -> cmp::Ordering {
		self.0.cmp(&rhs.0)
	}
}
#[cfg(feature = "serde")]
impl<T: ?Sized> serde::Serialize for Ptr64<T> {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		serializer.serialize_newtype_struct("Ptr64", &self.0)
	}
}

unsafe impl<T: ?Sized> Pod for Ptr64<T> {}

//----------------------------------------------------------------

#[cfg(test)]
mod tests {
	use std::mem;
	use super::*;

	#[test]
	fn ptr64() {
		let a = Ptr64::<f64>::from(0x2000);
		let b = a + 0x40;
		let c = a - 0x40;
		assert_eq!(mem::size_of_val(&a), 8);
		assert_eq!(c - a, -0x40);
		assert_eq!(b.into_raw(), 0x2200);
		assert_eq!(format!("{}", a), "0x0000000000002000");
		assert_eq!(c.into_raw(), 0x1E00);
	}
}
