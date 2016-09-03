/*!
Screenshot test.

Captures the center 256x256 rect of the foreground window.
*/

#![allow(unused_variables)]

use ::std::fs;

extern crate external;
use ::external::snap;
use ::external::window::{Window};

fn main() {
	// Create the capture context, allocating the destination buffer
	// Note that you can specify a sub rectangle to capture with `snap::Capture::with_rect`
	let capture = snap::Capture::new(Window::foreground().unwrap()).unwrap();
	println!("{:?}", capture);
	// `BitBlt` all the pixels
	capture.blit().unwrap();
	// Create storage for the pixels
	let mut image = snap::Image::default();
	capture.pixels(&mut image).unwrap();
	// Save to file as `.ppm` V6
	let mut file = fs::File::open("ss.ppm").unwrap();
	image.save(&mut file).unwrap();
}
