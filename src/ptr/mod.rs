/*!
Remote pointers.

# Rationale

Why is this abstraction useful, why not just use Rust's raw pointers?

These pointers point to memory in other processes; they are not valid within the process they are used from.
The pointers may not be of the same size, eg. 64bit process with a pointer to a 32bit process.

A decision was made to not support lifetimes, these are effectively raw pointers.
Reading and writing across processes at least prevent reading from invalid memory although the result may not be as expected if it happens to be reused for a different datastructure.

# Pointer types

There is both a raw pointer type and a typed pointer type.

Typed pointers allow the type system to assist you in preventing mistakes when interacting with this memory.

# Operations

All the pointer types implement these interfaces:

* Conversion between the underlying unsigned integer type and back to pointer type.

* Conversion between the raw and typed pointer types.

* Difference between two pointers of the same type; for raw pointers the difference is in bytes and for typed pointers the difference is in the number of elements between them.

* Adding and subtracting an unsigned integer offset resulting in the same pointer with specified offset. For typed pointers the addition is in number of elements.

* Display and Debug formatting.
!*/

mod ptr64;
mod ptr32;

pub use self::ptr64::Ptr64;
pub use self::ptr32::Ptr32;

#[cfg(target_pointer_width = "64")]
pub type Ptr<T> = Ptr64<T>;

#[cfg(target_pointer_width = "32")]
pub type Ptr<T> = Ptr32<T>;

mod pod;
pub use self::pod::Pod;

impl<T: ?Sized> From<Ptr32<T>> for Ptr64<T> {
	fn from(ptr: Ptr32<T>) -> Ptr64<T> {
		Ptr64::from(ptr.into_raw() as u64)
	}
}
