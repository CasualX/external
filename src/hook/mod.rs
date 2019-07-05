/*!
Windows hooks.

The most important thing to know is that the callbacks are context-less.

You cannot pass a `self` of any kind to communicate to the outside world, the only way to get information out is through global mutable state.
This is an API design limitation of `SetWindowsHookEx` itself.

# Examples

Use the provided `windows_hook!` to create the hooks instead of implementing the traits manually.

This complexity is needed due to the lack of context pointer making the `Fn*` traits not usable.

```
# #[macro_use] extern crate external; fn main() {
windows_hook! {
	/// A function with the given name which takes no arguments is created.
	/// This function registers the hook and returns the registration result.
	/// Doc comments, other attributes and optional `pub` will be applied to this function.
	///
	/// The callback type is defined by the argument identifier:
	/// * `KeyboardLL` means this is a low level keyboard hook.
	/// * `MouseLL` means this is a low level mouse hook.
	pub fn my_hook(context: &mut external::hook::KeyboardLL) {
		println!("{:?}", context);
	}
}
# }
```

Generates the following code:

```
/// {{doc-comment}}
pub fn my_hook() -> Result<external::hook::Hook, external::error::ErrorCode> {
	enum T {}
	impl external::hook::WindowsHook for T {
		type Context = external::hook::KeyboardLL;
		fn invoke(context: &mut external::hook::KeyboardLL) {
			println!("{:?}", context);
		}
	}
	<T as external::hook::WindowsHook>::register()
}
```

Register the hook by simply calling the defined function and unwrapping it.
!*/

use std::{ptr};
use crate::error::ErrorCode;
use crate::winapi::*;

pub unsafe trait HookContext: Sized {
	/// The windows idHook type.
	fn hook_type() -> c_int;
	/// Construct the context from its raw parameters.
	unsafe fn from_raw(code: c_int, w_param: WPARAM, l_param: LPARAM) -> Self;
	/// Invokes the next hook with the right parameters.
	unsafe fn call_next_hook(&self) -> LRESULT;
}

/// User callbacks.
pub trait WindowsHook: Sized {
	/// The type of callback.
	type Context: HookContext;
	/// The callback to invoke.
	///
	/// # Safety
	///
	/// Do not move the context out of the `&mut` reference.
	/// It contains pointers internally that will not outlive the invoke callback.
	fn invoke(arg: &mut Self::Context);
	/// Unsafe thunk to your Rust callback.
	unsafe extern "system" fn thunk(code: c_int, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
		let mut context = Self::Context::from_raw(code, w_param, l_param);
		if code >= 0 {
			Self::invoke(&mut context);
		}
		context.call_next_hook()
	}
	/// Registers the hook.
	fn register() -> Result<Hook, ErrorCode> {
		unsafe {
			let hook = SetWindowsHookExW(Self::Context::hook_type(), Some(Self::thunk), ptr::null_mut(), 0);
			if hook.is_null() {
				Err(ErrorCode::last())
			}
			else {
				Ok(Hook(hook))
			}
		}
	}
}

/// Setup a windows hook callback.
///
/// See the [hook module](hook/index.html)'s documentation for more information.
#[macro_export]
macro_rules! windows_hook {
	(
		$(#[$meta:meta])*
		$vis:vis fn $name:ident($arg:ident: &mut $ty:ty) $body:tt
	) => {
		$(#[$meta])*
		$vis fn $name() -> Result<$crate::hook::Hook, $crate::error::ErrorCode> {
			enum T {}
			impl $crate::hook::WindowsHook for T {
				type Context = $ty;
				fn invoke($arg: &mut $ty) $body
			}
			<T as $crate::hook::WindowsHook>::register()
		}
	};
}

/// The hook registration.
///
/// The hook is unhooked when this instance goes out of scope.
pub struct Hook(HHOOK);
impl Drop for Hook {
	fn drop(&mut self) {
		unsafe {
			UnhookWindowsHookEx(self.0);
		}
	}
}

mod keyboard_ll;
pub use self::keyboard_ll::*;

mod mouse_ll;
pub use self::mouse_ll::*;
