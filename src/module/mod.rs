/*!
Modules.
!*/

mod module_enum;
mod module_ldr_data;

pub use self::module_enum::*;
pub use self::module_ldr_data::*;

extern "C" {
	static __ImageBase: u8;
}

/// Returns this module's image base.
pub fn image_base() -> crate::winapi::HMODULE {
	unsafe { &__ImageBase as *const _ as crate::winapi::HMODULE }
}
