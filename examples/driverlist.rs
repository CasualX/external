/*!
Lists drivers which can register callbacks.
!*/

use external::system::SystemModules;

fn main() {
	println!("Drivers with 0x20 flag set:");
	for sm in &SystemModules::query() {
		if sm.flags() & 0x20 != 0 {
			println!("{:#018x}: {:#?}", sm.image_base(), sm.file_name());
		}
	}
}
