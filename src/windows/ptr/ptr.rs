
use std::{cmp, fmt, ops, mem};
use std::marker::PhantomData;

use super::{Pod, RawPtr32, TypePtr32};

//----------------------------------------------------------------

/// Raw Pointer.
#[derive(Copy, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(C)]
pub struct RawPtr(usize);
impl From<usize> for RawPtr {
	fn from(addr: usize) -> RawPtr {
		RawPtr(addr)
	}
}
impl From<RawPtr> for usize {
	fn from(ptr: RawPtr) -> usize {
		ptr.0
	}
}
impl<T: ?Sized> From<TypePtr<T>> for RawPtr {
	fn from(ptr: TypePtr<T>) -> RawPtr {
		RawPtr(ptr.0)
	}
}
impl From<RawPtr32> for RawPtr {
	fn from(ptr: RawPtr32) -> RawPtr {
		RawPtr(ptr.into())
	}
}
impl<T: ?Sized> From<TypePtr32<T>> for RawPtr {
	fn from(ptr: TypePtr32<T>) -> RawPtr {
		RawPtr(ptr.into())
	}
}
impl RawPtr {
	pub fn null() -> RawPtr {
		RawPtr(0)
	}
	pub fn is_null(self) -> bool {
		self.0 == 0
	}
}
impl ops::Sub for RawPtr {
	type Output = i64;
	fn sub(self, rhs: RawPtr) -> i64 {
		usize::wrapping_sub(self.0, rhs.0) as i64
	}
}
impl fmt::Display for RawPtr {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:>#018X}", self.0)
	}
}
impl fmt::Debug for RawPtr {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "RawPtr({:>#018X})", self.0)
	}
}
impl ops::Add<usize> for RawPtr {
	type Output = RawPtr;
	fn add(self, rhs: usize) -> RawPtr {
		RawPtr(self.0 + rhs)
	}
}
impl ops::Sub<usize> for RawPtr {
	type Output = RawPtr;
	fn sub(self, rhs: usize) -> RawPtr {
		RawPtr(self.0 - rhs)
	}
}
unsafe impl Pod for RawPtr {}

//----------------------------------------------------------------

/// 64bit Typed Pointer.
#[repr(C)]
pub struct TypePtr<T: ?Sized>(usize, PhantomData<*mut T>);
impl<T: ?Sized> From<usize> for TypePtr<T> {
	fn from(addr: usize) -> TypePtr<T> {
		TypePtr(addr, PhantomData)
	}
}
impl<T: ?Sized> From<TypePtr<T>> for usize {
	fn from(ptr: TypePtr<T>) -> usize {
		ptr.0
	}
}
impl<T: ?Sized> From<RawPtr> for TypePtr<T> {
	fn from(ptr: RawPtr) -> TypePtr<T> {
		TypePtr(ptr.0, PhantomData)
	}
}
impl<T: ?Sized> From<RawPtr32> for TypePtr<T> {
	fn from(ptr: RawPtr32) -> TypePtr<T> {
		TypePtr(ptr.into(), PhantomData)
	}
}
impl<T: ?Sized> From<TypePtr32<T>> for TypePtr<T> {
	fn from(ptr: TypePtr32<T>) -> TypePtr<T> {
		TypePtr(ptr.into(), PhantomData)
	}
}
impl<T: ?Sized> TypePtr<T> {
	pub fn null() -> TypePtr<T> {
		TypePtr(0, PhantomData)
	}
	pub fn is_null(self) -> bool {
		self.0 == 0
	}
}
impl<T> ops::Sub for TypePtr<T> {
	type Output = i64;
	fn sub(self, rhs: TypePtr<T>) -> i64 {
		(usize::wrapping_sub(self.0, rhs.0) as i64) / mem::size_of::<T>() as i64
	}
}
impl<T: ?Sized> fmt::Display for TypePtr<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:>#018X}", self.0)
	}
}
impl<T: ?Sized> fmt::Debug for TypePtr<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "TypePtr({:>#018X})", self.0)
	}
}
impl<T> ops::Add<usize> for TypePtr<T> {
	type Output = TypePtr<T>;
	fn add(self, rhs: usize) -> TypePtr<T> {
		TypePtr(self.0 + rhs * mem::size_of::<T>() as usize, self.1)
	}
}
impl<T> ops::Sub<usize> for TypePtr<T> {
	type Output = TypePtr<T>;
	fn sub(self, rhs: usize) -> TypePtr<T> {
		TypePtr(self.0 - rhs * mem::size_of::<T>() as usize, self.1)
	}
}
impl<T: ?Sized> Clone for TypePtr<T> {
	fn clone(&self) -> Self {
		TypePtr(self.0, self.1)
	}
}
impl<T: ?Sized> Default for TypePtr<T> {
	fn default() -> TypePtr<T> {
		TypePtr::null()
	}
}
impl<T: ?Sized> PartialEq for TypePtr<T> {
	fn eq(&self, rhs: &Self) -> bool {
		self.0 == rhs.0
	}
}
impl<T: ?Sized> PartialOrd for TypePtr<T> {
	fn partial_cmp(&self, rhs: &TypePtr<T>) -> Option<cmp::Ordering> {
		self.0.partial_cmp(&rhs.0)
	}
}
impl<T: ?Sized> Copy for TypePtr<T> {}
impl<T: ?Sized> Eq for TypePtr<T> {}
impl<T: ?Sized> Ord for TypePtr<T> {
	fn cmp(&self, rhs: &TypePtr<T>) -> cmp::Ordering {
		self.0.cmp(&rhs.0)
	}
}
unsafe impl<T: ?Sized + Pod> Pod for TypePtr<T> {}

//----------------------------------------------------------------

#[cfg(test)]
mod tests {
	use ::std::mem;
	use super::*;

	#[test]
	fn rawptr64() {
		let a = RawPtr::from(0x1000);
		let b = a + 0x20;
		let c = a - 0x20;
		assert_eq!(mem::size_of_val(&a), 8);
		assert_eq!(b - a, 0x20);
		assert_eq!({ let ptr: usize = c.into(); ptr }, 0x0FE0);
		assert_eq!(format!("{}", a), "0x0000000000001000");
		assert_eq!({ let ptr: TypePtr<i64> = b.into(); ptr }, TypePtr::<i64>::from(0x1020));
	}

	#[test]
	fn typeptr64() {
		let a = TypePtr::<f64>::from(0x2000);
		let b = a + 0x40;
		let c = a - 0x40;
		assert_eq!(mem::size_of_val(&a), 8);
		assert_eq!(c - a, -0x40);
		assert_eq!({ let ptr: usize = b.into(); ptr }, 0x2200);
		assert_eq!(format!("{}", a), "0x0000000000002000");
		assert_eq!({ let ptr: RawPtr = c.into(); ptr }, RawPtr::from(0x1E00));
	}
}
