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

pub use intptr::IntPtr32 as Ptr32;
pub use intptr::IntPtr64 as Ptr64;
pub use intptr::IntPtr as Ptr;

pub use dataview::Pod;
