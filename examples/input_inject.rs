/*!
Inject a SPACE keypress.
*/

extern crate external;

use ::std::{thread, time};
use ::external::input;

fn main() {
	input::key_down(input::vk::SPACE);
	thread::sleep(time::Duration::from_millis(100));
	input::key_up(input::vk::SPACE);
}
