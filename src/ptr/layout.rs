/*
/// Implements explicit field access for a struct with explicit layout.
///
/// This macro implements the accessor method with compiletime checks:
/// * Both the struct and the field type must be `Pod`.
/// * The offset plus field size must be less than or equal to the size of the struct.
/// * When using a `ref` or `mut` accessor the field and struct must have compatible alignment.
///
/// Examples
/// ========
///
/// Define the structure as a newtype wrapper around an array.
/// The element type of the array controls the struct's alignment and size.
/// Recommended types are one of `u8`, `u16`, `u32` or `u64`.
///
/// ```
/// #[derive(Copy, Clone)]
/// pub struct ExplicitLayout([u32; 4]);
/// ```
///
/// Then implement any field accesses inside an `impl` block on the structure.
///
/// ```
/// # struct ExplicitLayout([u32; 4]);
/// impl ExplicitLayout {
/// 	external::explicit_field! {
/// 		pub ref field {+0x10}: f32,
/// 	}
/// }
/// ```
///
/// There's a lot to unpack here:
///
/// Any attributes, doc strings and visibility modifiers are applied to the accessor method.
///
/// Followed by the type of accessor method.
/// Unfortunately it is not required to manually define the kind of accessor as macros cannot yet generate the names.
/// The following type of accessor are supported:
/// * `ref` returns a shared borrow to the field in the structure.
/// * `mut` returns a unique borrow to the field in the structure.
/// * `get` returns the field by value using unaligned read.
/// * `set` sets the field's value using unaligned write.
///
/// The name of the field is followed by `{+offset}` where offset is a const expression
/// evaluating to the offset in the structure where the field is located.
///
/// The field specified by the offset must be contained within the structure.
/// For `mut` or `ref` fields, the field must be compatible with the field and structure alignment requirements.
/// Failure to adhere to these constraints will result in compiletime error.
///
/// Finally, the trailing comma is optional.
#[macro_export]
macro_rules! explicit_field {
	(
		$(#[$field_meta:meta])*
		$field_vis:vis ref $field_name:ident {+$field_offset:expr}: $field_ty:ty $(,)?
	) => {
		$(#[$field_meta])*
		$field_vis fn $field_name(&self) -> &$field_ty where Self: $crate::ptr::Pod, $field_ty: $crate::ptr::Pod {
			use ::std::mem;
			const FIELD_OFFSET: usize = $field_offset;
			// Make some static assertions about the inputs
			const INBOUNDS: bool = FIELD_OFFSET + mem::size_of::<$field_ty>() <= mem::size_of::<Self>();
			const ALIGNED1: bool = FIELD_OFFSET % mem::align_of::<$field_ty>() == 0;
			const ALIGNED2: bool = mem::align_of::<Self>() % mem::align_of::<$field_ty>() == 0;
			let _: [[[(); INBOUNDS as usize - 1]; ALIGNED1 as usize - 1]; ALIGNED2 as usize - 1];
			// Given all that, this is probably safe?
			unsafe {
				&*((self as *const _ as *const u8).offset(FIELD_OFFSET as isize) as *const $field_ty)
			}
		}
	};
	(
		$(#[$field_meta:meta])*
		$field_vis:vis mut $field_name:ident {+$field_offset:expr}: $field_ty:ty
	) => {
		$(#[$field_meta])*
		$field_vis fn $field_name(&mut self) -> &mut $field_ty where Self: $crate::ptr::Pod, $field_ty: $crate::ptr::Pod {
			use ::std::mem;
			const FIELD_OFFSET: usize = $field_offset;
			// Make some static assertions about the inputs
			const INBOUNDS: bool = FIELD_OFFSET + mem::size_of::<$field_ty>() <= mem::size_of::<Self>();
			const ALIGNED1: bool = FIELD_OFFSET % mem::align_of::<$field_ty>() == 0;
			const ALIGNED2: bool = mem::align_of::<Self>() % mem::align_of::<$field_ty>() == 0;
			let _: [[[(); INBOUNDS as usize - 1]; ALIGNED1 as usize - 1]; ALIGNED2 as usize - 1];
			// Given all that, this is probably safe?
			unsafe {
				&mut *((self as *mut _ as *mut u8).offset(FIELD_OFFSET as isize) as *mut $field_ty)
			}
		}
	};
	(
		$(#[$field_meta:meta])*
		$field_vis:vis get $field_name:ident {+$field_offset:expr}: $field_ty:ty
	) => {
		$(#[$field_meta])*
		$field_vis fn $field_name(&self) -> $field_ty where Self: $crate::ptr::Pod, $field_ty: $crate::ptr::Pod {
			use ::std::{mem, ptr};
			const FIELD_OFFSET: usize = $field_offset;
			// Static assert $field_offset points within the structure
			let _: [(); (FIELD_OFFSET + mem::size_of::<$field_ty>() <= mem::size_of::<Self>()) as usize - 1];
			// Given all that, this is probably safe?
			unsafe {
				ptr::read_unaligned((self as *const _ as *const u8).offset(FIELD_OFFSET as isize) as *const $field_ty)
			}
		}
	};
	(
		$(#[$field_meta:meta])*
		$field_vis:vis set $field_name:ident {+$field_offset:expr}: $field_ty:ty
	) => {
		$(#[$field_meta])*
		$field_vis fn $field_name(&mut self, value: $field_ty) where Self: $crate::ptr::Pod, $field_ty: $crate::ptr::Pod {
			use ::std::{mem, ptr};
			const FIELD_OFFSET: usize = $field_offset;
			// Static assert $field_offset points within the structure
			let _: [(); (FIELD_OFFSET + mem::size_of::<$field_ty>() <= mem::size_of::<Self>()) as usize - 1];
			// Given all that, this is probably safe?
			unsafe {
				ptr::write_unaligned((self as *mut _ as *mut u8).offset(FIELD_OFFSET as isize) as *mut $field_ty, value);
			}
		}
	};
}
*/

/// Creates a struct with explicit layout.
///
/// # Examples
///
/// ```
/// external::explicit_struct! {
/// 	/// Docstring for the structure.
/// 	pub struct ExplicitStruct(pub [u64; 8]);
/// 	pub field {+4}: f32,
/// 	unk01 {+8}: u8,
/// 	pub another {+9}: u8,
/// }
/// ```
#[macro_export]
macro_rules! explicit_struct {
	(
		$(#[$struct_meta:meta])*
		$struct_vis:vis struct $struct_name:ident ($array_vis:vis $array_type:ty);
		$(
			$(#[$field_meta:meta])*
			$field_vis:vis $field_name:ident {+$field_offset:expr}: $field_ty:ty,
		)*
	) => {
		$(#[$struct_meta])*
		#[repr(transparent)]
		$struct_vis struct $struct_name($array_vis $array_type) where $array_type: $crate::ptr::Pod;

		unsafe impl $crate::ptr::Pod for $struct_name where $array_type: $crate::ptr::Pod {}

		impl ::std::default::Default for $struct_name {
			fn default() -> $struct_name {
				$struct_name(unsafe { ::std::mem::zeroed() })
			}
		}

		impl $struct_name {
			pub unsafe fn uninit() -> $struct_name {
				$struct_name(::std::mem::uninitialized())
			}
		}
		impl $struct_name {
			$(
				$(#[$field_meta])*
				$field_vis fn $field_name(&self) -> &$field_ty where $struct_name: $crate::ptr::Pod, $field_ty: $crate::ptr::Pod {
					use ::std::mem;
					const FIELD_OFFSET: usize = $field_offset;
					// Make some static assertions about the inputs
					const INBOUNDS: bool = FIELD_OFFSET + mem::size_of::<$field_ty>() <= mem::size_of::<$struct_name>();
					const ALIGNED1: bool = FIELD_OFFSET % mem::align_of::<$field_ty>() == 0;
					const ALIGNED2: bool = mem::align_of::<$struct_name>() % mem::align_of::<$field_ty>() == 0;
					let _: [[[(); INBOUNDS as usize - 1]; ALIGNED1 as usize - 1]; ALIGNED2 as usize - 1];
					// Given all that, this is probably safe?
					unsafe {
						&*((self as *const _ as *const u8).offset(FIELD_OFFSET as isize) as *const $field_ty)
					}
				}
			)*
		}

		impl ::std::fmt::Debug for $struct_name where $($field_ty: ::std::fmt::Debug),* {
			fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
				f.debug_struct(stringify!($struct_name))
					$(.field(stringify!($field_name), &self.$field_name()))*
					.finish()
			}
		}
	};
}
