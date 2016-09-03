
/// Expose the raw inner type.
pub trait AsInner<T: ?Sized> {
	/// Exposes a reference to the raw inner type.
	fn as_inner(&self) -> &T;
}
/// Expose the mutable raw inner type.
pub trait AsInnerMut<T: ?Sized>: AsInner<T> {
	/// Exposes a mutable reference to the raw inner type.
	///
	/// # Safety
	///
	/// This is unsafe as it allows you to meddle with the inner bits without enforcing its invariants.
	unsafe fn as_inner_mut(&mut self) -> &mut T;
}

/// Convert into raw inner type.
pub trait IntoInner<T> {
	/// Returns the raw type for an idiomatic wrapper type.
	///
	/// The caller is responsible for cleaning up any owned resources.
	fn into_inner(self) -> T;
}
/// Convert from raw inner type.
pub trait FromInner<T> {
	/// Creates the idiomatic wrapper for a raw type.
	///
	/// # Safety
	///
	/// This is unsafe as any invariants of the raw type aren't checked before conversion.
	unsafe fn from_inner(inner: T) -> Self;
}

/// Quickly implement The `*Inner` traits for a newtype wrapper.
macro_rules! impl_inner {
	($ty:path: $inner:ty) => {
		impl $crate::inner::AsInner<$inner> for $ty {
			fn as_inner(&self) -> &$inner { &self.0 }
		}
		impl $crate::inner::AsInnerMut<$inner> for $ty {
			unsafe fn as_inner_mut(&mut self) -> &mut $inner { &mut self.0 }
		}
		impl $crate::inner::IntoInner<$inner> for $ty {
			fn into_inner(self) -> $inner { self.0 }
		}
		impl $crate::inner::FromInner<$inner> for $ty {
			unsafe fn from_inner(inner: $inner) -> $ty { $ty(inner) }
		}
	}
}
