/*!
Set the console control handler.

See [SetConsoleCtrlHandler function](https://msdn.microsoft.com/en-us/library/windows/desktop/ms686016.aspx) for more information.

# Examples

```
use external::control::{CtrlHandler, CtrlEvent};

pub struct MyHandler;
impl CtrlHandler for MyHandler {
	fn control(event: CtrlEvent) -> bool {
		// If the event is handled return `true`
		// to stop the next handler function from being called.
		false
	}
}

// Add the ctrl handler.
let _guard = MyHandler::add().unwrap();

// The ctrl handler is removed when the guard is dropped.
```
*/

use winapi::um::consoleapi::{SetConsoleCtrlHandler};
use winapi::um::wincon::{CTRL_C_EVENT, CTRL_BREAK_EVENT, CTRL_CLOSE_EVENT, CTRL_LOGOFF_EVENT, CTRL_SHUTDOWN_EVENT};
use winapi::shared::minwindef::{BOOL, FALSE, TRUE, DWORD};

use crate::error::{ErrorCode};
use crate::Result;

//----------------------------------------------------------------

/// The type of control signal received by the handler.
///
/// See [HandlerRoutine callback function](https://msdn.microsoft.com/en-us/library/windows/desktop/ms683242.aspx) for more information.
pub enum CtrlEvent {
	C,
	Break,
	Close,
	LogOff,
	Shutdown,
}

/// Defines the ctrl handler.
pub trait CtrlHandler: Sized {
	/// Add the ctrl handler.
	///
	/// See [SetConsoleCtrlHandler function](https://msdn.microsoft.com/en-us/library/windows/desktop/ms686016.aspx) for more information.
	fn add() -> Result<CtrlGuard> {
		unsafe {
			if SetConsoleCtrlHandler(Some(Self::thunk), TRUE) != FALSE {
				Ok(CtrlGuard(Some(Self::thunk)))
			}
			else {
				Err(ErrorCode::last())
			}
		}
	}

	/// The handler callback is called when a ctrl event is received.
	///
	/// See [HandlerRoutine callback function](https://msdn.microsoft.com/en-us/library/windows/desktop/ms683242.aspx) for more information.
	fn control(event: CtrlEvent) -> bool;

	#[doc(hidden)]
	unsafe extern "system" fn thunk(ctrl_type: DWORD) -> BOOL {
		let event = match ctrl_type {
			CTRL_C_EVENT => CtrlEvent::C,
			CTRL_BREAK_EVENT => CtrlEvent::Break,
			CTRL_CLOSE_EVENT => CtrlEvent::Close,
			CTRL_LOGOFF_EVENT => CtrlEvent::LogOff,
			CTRL_SHUTDOWN_EVENT => CtrlEvent::Shutdown,
			_ => return FALSE,
		};
		// FIXME! `catch_unwind` me?
		if Self::control(event) { TRUE } else { FALSE }
	}
}

/// Guard removes the ctrl handler.
pub struct CtrlGuard(Option<unsafe extern "system" fn(DWORD) -> BOOL>);
impl Drop for CtrlGuard {
	fn drop(&mut self) {
		unsafe {
			SetConsoleCtrlHandler(self.0, FALSE);
		}
	}
}

//----------------------------------------------------------------

/// Sets the ignore handler routine.
pub struct CtrlIgnore;
impl CtrlHandler for CtrlIgnore {
	fn add() -> Result<CtrlGuard> {
		unsafe {
			if SetConsoleCtrlHandler(None, TRUE) != FALSE {
				Ok(CtrlGuard(None))
			}
			else {
				Err(ErrorCode::last())
			}
		}
	}
	fn control(_: CtrlEvent) -> bool {
		false
	}
}
