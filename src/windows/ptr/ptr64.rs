
use std::{cmp, fmt, ops, mem};
use std::marker::PhantomData;

use super::Pod;

//----------------------------------------------------------------

/// Raw Pointer.
#[derive(Copy, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(C)]
pub struct RawPtr64(u64);

impl From<u64> for RawPtr64 {
	fn from(address: u64) -> RawPtr64 {
		RawPtr64(address)
	}
}
impl From<RawPtr64> for u64 {
	fn from(ptr: RawPtr64) -> u64 {
		ptr.0
	}
}

impl<T: ?Sized> From<TypePtr64<T>> for RawPtr64 {
	fn from(ptr: TypePtr64<T>) -> RawPtr64 {
		RawPtr64(ptr.0)
	}
}

impl RawPtr64 {
	/// Returns a raw null pointer.
	pub fn null() -> RawPtr64 {
		RawPtr64(0)
	}
	/// Returns if the pointer is the null pointer.
	pub fn is_null(self) -> bool {
		self.0 == 0
	}
	/// Converts to a `u64` value.
	pub fn into_u64(self) -> u64 {
		self.0
	}
	/// Converts to a typed pointer.
	pub fn into_typed<T: ?Sized>(self) -> TypePtr64<T> {
		TypePtr64(self.0, PhantomData)
	}
}
impl ops::Sub for RawPtr64 {
	type Output = i64;
	fn sub(self, rhs: RawPtr64) -> i64 {
		u64::wrapping_sub(self.0, rhs.0) as i64
	}
}
impl fmt::Display for RawPtr64 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:>#018X}", self.0)
	}
}
impl fmt::Debug for RawPtr64 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "RawPtr64({:>#018X})", self.0)
	}
}
impl ops::Add<u64> for RawPtr64 {
	type Output = RawPtr64;
	fn add(self, rhs: u64) -> RawPtr64 {
		RawPtr64(self.0 + rhs)
	}
}
impl ops::Sub<u64> for RawPtr64 {
	type Output = RawPtr64;
	fn sub(self, rhs: u64) -> RawPtr64 {
		RawPtr64(self.0 - rhs)
	}
}

//----------------------------------------------------------------

/// 64bit Typed Pointer.
#[repr(C)]
pub struct TypePtr64<T: ?Sized>(u64, PhantomData<fn() -> T>);

impl<T: ?Sized> From<u64> for TypePtr64<T> {
	fn from(addr: u64) -> TypePtr64<T> {
		TypePtr64(addr, PhantomData)
	}
}
impl<T: ?Sized> From<TypePtr64<T>> for u64 {
	fn from(ptr: TypePtr64<T>) -> u64 {
		ptr.0
	}
}
impl<T: ?Sized> From<RawPtr64> for TypePtr64<T> {
	fn from(ptr: RawPtr64) -> TypePtr64<T> {
		TypePtr64(ptr.0, PhantomData)
	}
}

impl<T: ?Sized> TypePtr64<T> {
	/// Returns a raw null pointer.
	pub fn null() -> TypePtr64<T> {
		TypePtr64(0, PhantomData)
	}
	/// Returns if the pointer is the null pointer.
	pub fn is_null(self) -> bool {
		self.0 == 0
	}
	/// Converts to a `u64` value.
	pub fn into_u64(self) -> u64 {
		self.0
	}
	/// Converts to a raw pointer.
	pub fn into_raw(self) -> RawPtr64 {
		RawPtr64(self.0)
	}
}
impl<T> TypePtr64<[T]> {
	pub fn decay(self) -> TypePtr64<T> {
		TypePtr64(self.0, PhantomData)
	}
	pub fn at(self, index: usize) -> TypePtr64<T> {
		TypePtr64(self.0 + mem::size_of::<T>() as u64 * index as u64, PhantomData)
	}
}
impl<T> ops::Sub for TypePtr64<T> {
	type Output = i64;
	fn sub(self, rhs: TypePtr64<T>) -> i64 {
		(u64::wrapping_sub(self.0, rhs.0) as i64) / mem::size_of::<T>() as i64
	}
}
impl<T: ?Sized> fmt::Display for TypePtr64<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:>#018X}", self.0)
	}
}
impl<T: ?Sized> fmt::Debug for TypePtr64<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "TypePtr64({:>#018X})", self.0)
	}
}
impl<T> ops::Add<u64> for TypePtr64<T> {
	type Output = TypePtr64<T>;
	fn add(self, rhs: u64) -> TypePtr64<T> {
		TypePtr64(self.0 + rhs * mem::size_of::<T>() as u64, self.1)
	}
}
impl<T> ops::Sub<u64> for TypePtr64<T> {
	type Output = TypePtr64<T>;
	fn sub(self, rhs: u64) -> TypePtr64<T> {
		TypePtr64(self.0 - rhs * mem::size_of::<T>() as u64, self.1)
	}
}
impl<T: ?Sized> Clone for TypePtr64<T> {
	fn clone(&self) -> Self {
		TypePtr64(self.0, self.1)
	}
}
impl<T: ?Sized> Default for TypePtr64<T> {
	fn default() -> TypePtr64<T> {
		TypePtr64::null()
	}
}
impl<T: ?Sized> PartialEq for TypePtr64<T> {
	fn eq(&self, rhs: &Self) -> bool {
		self.0 == rhs.0
	}
}
impl<T: ?Sized> PartialOrd for TypePtr64<T> {
	fn partial_cmp(&self, rhs: &TypePtr64<T>) -> Option<cmp::Ordering> {
		self.0.partial_cmp(&rhs.0)
	}
}
impl<T: ?Sized> Copy for TypePtr64<T> {}
impl<T: ?Sized> Eq for TypePtr64<T> {}
impl<T: ?Sized> Ord for TypePtr64<T> {
	fn cmp(&self, rhs: &TypePtr64<T>) -> cmp::Ordering {
		self.0.cmp(&rhs.0)
	}
}

unsafe impl Pod for RawPtr64 {}
unsafe impl<T: ?Sized> Pod for TypePtr64<T> {}

//----------------------------------------------------------------

#[cfg(test)]
mod tests {
	use std::mem;
	use super::*;

	#[test]
	fn rawptr64() {
		let a = RawPtr64::from(0x1000);
		let b = a + 0x20;
		let c = a - 0x20;
		assert_eq!(mem::size_of_val(&a), 8);
		assert_eq!(b - a, 0x20);
		assert_eq!(c.into_u64(), 0x0FE0);
		assert_eq!(format!("{}", a), "0x0000000000001000");
		assert_eq!(c.into_typed::<i64>(), TypePtr64::<i64>::from(0x1020));
	}

	#[test]
	fn typeptr64() {
		let a = TypePtr64::<f64>::from(0x2000);
		let b = a + 0x40;
		let c = a - 0x40;
		assert_eq!(mem::size_of_val(&a), 8);
		assert_eq!(c - a, -0x40);
		assert_eq!(b.into_u64(), 0x2200);
		assert_eq!(format!("{}", a), "0x0000000000002000");
		assert_eq!(c.into_raw(), RawPtr64::from(0x1E00));
	}
}
