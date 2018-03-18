/*!
Lists drivers and if they can register callbacks.
*/

extern crate external;

use external::system::SystemModules;

fn main() {
	println!("List of drivers with 0x20 flag set:");
	for sm in &SystemModules::query() {
		if sm.flags() & 0x20 != 0 {
			println!("{:#?}", sm.file_name());
		}
	}
}
