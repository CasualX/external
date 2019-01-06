
use std::{cmp, fmt, ops, mem};
use std::marker::PhantomData;

use super::Pod;

//----------------------------------------------------------------

/// 32bit Raw Pointer.
#[derive(Copy, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(C)]
pub struct RawPtr32(u32);

impl From<u32> for RawPtr32 {
	fn from(address: u32) -> RawPtr32 {
		RawPtr32(address)
	}
}
impl From<RawPtr32> for u32 {
	fn from(ptr: RawPtr32) -> u32 {
		ptr.0
	}
}

impl<T: ?Sized> From<TypePtr32<T>> for RawPtr32 {
	fn from(ptr: TypePtr32<T>) -> RawPtr32 {
		RawPtr32(ptr.0)
	}
}

impl RawPtr32 {
	/// Returns a raw null pointer.
	pub fn null() -> RawPtr32 {
		RawPtr32(0)
	}
	/// Returns if the pointer is the null pointer.
	pub fn is_null(self) -> bool {
		self.0 == 0
	}
	/// Returns the pointer as a `u32`.
	pub fn into_u32(self) -> u32 {
		self.0
	}
	/// Returns the pointer as a typed pointer.
	pub fn into_typed<T: ?Sized>(self) -> TypePtr32<T> {
		TypePtr32(self.0, PhantomData)
	}
}
impl ops::Sub for RawPtr32 {
	type Output = i32;
	fn sub(self, rhs: RawPtr32) -> i32 {
		u32::wrapping_sub(self.0, rhs.0) as i32
	}
}
impl fmt::Display for RawPtr32 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:>#010X}", self.0)
	}
}
impl fmt::Debug for RawPtr32 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "RawPtr32({:>#010X})", self.0)
	}
}
impl ops::Add<u32> for RawPtr32 {
	type Output = RawPtr32;
	fn add(self, rhs: u32) -> RawPtr32 {
		RawPtr32(self.0 + rhs)
	}
}
impl ops::Sub<u32> for RawPtr32 {
	type Output = RawPtr32;
	fn sub(self, rhs: u32) -> RawPtr32 {
		RawPtr32(self.0 - rhs)
	}
}

//----------------------------------------------------------------

/// 32bit Typed Pointer.
#[repr(C)]
pub struct TypePtr32<T: ?Sized>(u32, PhantomData<fn() -> T>);

impl<T: ?Sized> From<u32> for TypePtr32<T> {
	fn from(address: u32) -> TypePtr32<T> {
		TypePtr32(address, PhantomData)
	}
}
impl<T: ?Sized> From<TypePtr32<T>> for u32 {
	fn from(ptr: TypePtr32<T>) -> u32 {
		ptr.0
	}
}
impl<T: ?Sized> From<RawPtr32> for TypePtr32<T> {
	fn from(ptr: RawPtr32) -> TypePtr32<T> {
		TypePtr32(ptr.0, PhantomData)
	}
}

impl<T: ?Sized> TypePtr32<T> {
	/// Returns a raw null pointer.
	pub fn null() -> TypePtr32<T> {
		TypePtr32(0, PhantomData)
	}
	/// Returns if the pointer is the null pointer.
	pub fn is_null(self) -> bool {
		self.0 == 0
	}
	/// Returns the pointer as a `u32`.
	pub fn into_u32(self) -> u32 {
		self.0
	}
	/// Returns the pointer as a raw pointer.
	pub fn into_raw(self) -> RawPtr32 {
		RawPtr32(self.0)
	}
}
impl<T> TypePtr32<[T]> {
	pub fn decay(self) -> TypePtr32<T> {
		TypePtr32(self.0, PhantomData)
	}
	pub fn at(self, index: usize) -> TypePtr32<T> {
		TypePtr32(self.0 + mem::size_of::<T>() as u32 * index as u32, PhantomData)
	}
}
impl<T> ops::Sub for TypePtr32<T> {
	type Output = i32;
	fn sub(self, rhs: TypePtr32<T>) -> i32 {
		(u32::wrapping_sub(self.0, rhs.0) as i32) / mem::size_of::<T>() as i32
	}
}
impl<T: ?Sized> fmt::Display for TypePtr32<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:>#010X}", self.0)
	}
}
impl<T: ?Sized> fmt::Debug for TypePtr32<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "TypePtr32({:>#010X})", self.0)
	}
}
impl<T> ops::Add<u32> for TypePtr32<T> {
	type Output = TypePtr32<T>;
	fn add(self, rhs: u32) -> TypePtr32<T> {
		TypePtr32(self.0 + rhs * mem::size_of::<T>() as u32, self.1)
	}
}
impl<T> ops::Sub<u32> for TypePtr32<T> {
	type Output = TypePtr32<T>;
	fn sub(self, rhs: u32) -> TypePtr32<T> {
		TypePtr32(self.0 - rhs * mem::size_of::<T>() as u32, self.1)
	}
}
impl<T: ?Sized> Clone for TypePtr32<T> {
	fn clone(&self) -> Self {
		TypePtr32(self.0, self.1)
	}
}
impl<T: ?Sized> Default for TypePtr32<T> {
	fn default() -> TypePtr32<T> {
		TypePtr32::null()
	}
}
impl<T: ?Sized> PartialEq for TypePtr32<T> {
	fn eq(&self, rhs: &Self) -> bool {
		self.0 == rhs.0
	}
}
impl<T: ?Sized> PartialOrd for TypePtr32<T> {
	fn partial_cmp(&self, rhs: &TypePtr32<T>) -> Option<cmp::Ordering> {
		self.0.partial_cmp(&rhs.0)
	}
}
impl<T: ?Sized> Copy for TypePtr32<T> {}
impl<T: ?Sized> Eq for TypePtr32<T> {}
impl<T: ?Sized> Ord for TypePtr32<T> {
	fn cmp(&self, rhs: &TypePtr32<T>) -> cmp::Ordering {
		self.0.cmp(&rhs.0)
	}
}

unsafe impl Pod for RawPtr32 {}
unsafe impl<T: ?Sized> Pod for TypePtr32<T> {}

//----------------------------------------------------------------

#[cfg(test)]
mod tests {
	use std::mem;
	use super::*;

	#[test]
	fn rawptr32() {
		let a = RawPtr32::from(0x1000);
		let b = a + 0x20;
		let c = a - 0x20;
		assert_eq!(mem::size_of_val(&a), 4);
		assert_eq!(b - a, 0x20);
		assert_eq!(c.into_u32(), 0x0FE0);
		assert_eq!(format!("{}", a), "0x00001000");
		assert_eq!(b.into_typed::<i32>(), TypePtr32::<i32>::from(0x1020));
	}

	#[test]
	fn typeptr32() {
		let a = TypePtr32::<f32>::from(0x2000);
		let b = a + 0x40;
		let c = a - 0x40;
		assert_eq!(mem::size_of_val(&a), 4);
		assert_eq!(c - a, -0x40);
		assert_eq!(b.into_u32(), 0x2100);
		assert_eq!(format!("{}", a), "0x00002000");
		assert_eq!(c.into_raw(), RawPtr32::from(0x1F00));
	}
}
