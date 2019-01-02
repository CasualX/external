/*!
Screenshots using GDI.
*/

use std::{mem, ptr, io};

use winapi::um::winuser::{GetDC, ReleaseDC};
use winapi::um::wingdi::{DeleteDC, CreateCompatibleDC, CreateCompatibleBitmap, SelectObject, DeleteObject, BitBlt, GetDIBits, GetObjectW};
use winapi::um::wingdi::{BITMAP, BITMAPINFO, BITMAPINFOHEADER, DIB_RGB_COLORS, SRCCOPY};
use winapi::ctypes::{c_void};
use winapi::shared::windef::{HDC, HBITMAP};
use winapi::shared::minwindef::{DWORD};

use crate::window::Window;
use crate::error::ErrorCode;
use crate::{Result, IntoInner};

//----------------------------------------------------------------

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Color {
	pub blue: u8,
	pub green: u8,
	pub red: u8,
	pub undef: u8,
}
impl Default for Color {
	fn default() -> Color {
		Color {
			blue: 0,
			green: 0,
			red: 0,
			undef: 0,
		}
	}
}
impl PartialEq<Color> for Color {
	fn eq(&self, rhs: &Color) -> bool {
		self.blue == rhs.blue && self.green == rhs.green && self.red == rhs.red
	}
}

//----------------------------------------------------------------

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Rect {
	pub left: i32,
	pub top: i32,
	pub width: i32,
	pub height: i32,
}

//----------------------------------------------------------------

#[derive(Debug)]
struct Source {
	wnd: Window,
	hdc: HDC,
}

/// Capture context.
#[derive(Debug)]
pub struct Capture {
	source: Source,
	hdc: HDC,
	hbmp: HBITMAP,
	rect: Rect,
}
impl Drop for Capture {
	fn drop(&mut self) {
		unsafe {
			DeleteObject(self.hbmp as *mut c_void);
			DeleteDC(self.hdc);
			ReleaseDC(self.source.wnd.into_inner(), self.source.hdc);
		}
	}
}
impl Capture {
	/// Get the window captured from.
	pub fn window(&self) -> Window {
		self.source.wnd
	}
	/// Get the subrectangle captured.
	pub fn rect(&self) -> &Rect {
		&self.rect
	}
}
impl Capture {
	/// Create a new capture context for the entire window.
	pub fn new(wnd: Window) -> Result<Capture> {
		let (width, height) = wnd.client_area()?;
		Self::with_rect(wnd, Rect { left: 0, top: 0, width: width, height: height })
	}
	/// Create a new capture context for a subrectangle for the window.
	pub fn with_rect(wnd: Window, rect: Rect) -> Result<Capture> {
		unsafe {
			let src_hdc = GetDC(wnd.into_inner());
			if !src_hdc.is_null() {
				let dest_hdc = CreateCompatibleDC(src_hdc);
				if !dest_hdc.is_null() {
					let hbmp = CreateCompatibleBitmap(src_hdc, rect.width, rect.height);
					if !hbmp.is_null() {
						SelectObject(dest_hdc, hbmp as *mut c_void);
						return Ok(Capture {
							source: Source {
								wnd: wnd,
								hdc: src_hdc,
							},
							hdc: dest_hdc,
							hbmp: hbmp,
							rect: rect,
						});
					}
					DeleteDC(dest_hdc);
				}
				ReleaseDC(wnd.into_inner(), src_hdc);
			}
			Err(ErrorCode::last())
		}
	}
	pub fn info(&self) -> BITMAP {
		unsafe {
			let mut bitmap: BITMAP = mem::uninitialized();
			let size_of = mem::size_of::<BITMAP>() as i32;
			let returned = GetObjectW(self.hbmp as *mut c_void, size_of, &mut bitmap as *mut _ as *mut c_void);
			assert_eq!(returned, size_of);
			bitmap
		}
	}
	/// Capture the screen pixels.
	pub fn blit(&self) -> Result<()> {
		unsafe {
			if BitBlt(self.hdc, 0, 0, self.rect.width, self.rect.height, self.source.hdc, self.rect.left, self.rect.top, SRCCOPY) != 0 {
				Ok(())
			}
			else {
				Err(ErrorCode::last())
			}
		}
	}
	/// Get the captured pixels.
	pub fn pixels(&self, image: &mut Image) -> Result<()> {
		unsafe {
			// FIXME! `GetDIBits` writes a color table of 3 values: RED, GREEN, BLUE. Why?
			//        Temporarily fixed by just allocating some extra fields which are ignored...
			let mut bmidata: [u32; 24] = [0xDDDDDDDD; 24];
			let bmi: &mut BITMAPINFO = mem::transmute(&mut bmidata);
			// Query bitmap info header
			bmi.bmiHeader.biSize = mem::size_of::<BITMAPINFOHEADER>() as DWORD;
			bmi.bmiHeader.biBitCount = 0;
			if GetDIBits(self.hdc, self.hbmp, 0, self.rect.height as u32, ptr::null_mut(), bmi, DIB_RGB_COLORS) == 0 {
				return Err(ErrorCode::last());
			}
			// Reserve space for the dibits
			let len = bmi.bmiHeader.biWidth as usize * bmi.bmiHeader.biHeight as usize;
			if image.pixels.capacity() < len {
				let additional = len - image.pixels.capacity();
				image.pixels.reserve_exact(additional);
			}
			// Copy the dibits
			let bits = image.pixels.as_mut_ptr() as *mut c_void;
			if GetDIBits(self.hdc, self.hbmp, 0, self.rect.height as u32, bits, bmi, DIB_RGB_COLORS) == 0 {
				return Err(ErrorCode::last());
			}
			// Write the result
			image.pixels.set_len(len);
			image.width = self.rect.width;
			image.height = self.rect.height;
			Ok(())
		}
	}
}

//----------------------------------------------------------------

#[derive(PartialEq)]
pub struct Image {
	pixels: Vec<Color>,
	width: i32,
	height: i32,
}

impl Image {
	pub fn pixels(&self) -> &[Color] {
		&self.pixels
	}
	pub fn width(&self) -> i32 {
		self.width
	}
	pub fn height(&self) -> i32 {
		self.height
	}
	pub fn save(&self, file: &mut io::Write) -> io::Result<()> {
		writeln!(file, "P6 {} {} 255", self.width, self.height)?;
		for i in 0..self.pixels.len() {
			let Color { red, green, blue, .. } = self.pixels[i];
			let color = [red, green, blue];
			file.write_all(&color)?;
		}
		Ok(())
	}
	pub fn load(file: &mut io::BufRead) -> io::Result<Image> {
		let mut s = String::new();
		file.read_line(&mut s)?;
		let mut line = s.split_whitespace();
		let (width, height) = if let (Some(header), Some(width), Some(height), Some(depth), None) = (line.next(), line.next(), line.next(), line.next(), line.next()) {
			if header != "P6" || depth != "255" {
				return Err(io::Error::new(io::ErrorKind::InvalidData, "unknown file format"));
			}
			let height: usize = height.parse().map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;
			let width: usize = width.parse().map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;
			(width, height)
		}
		else {
			return Err(io::Error::new(io::ErrorKind::InvalidData, "unknown file format"));
		};
		let num = width * height;
		let mut pixels = Vec::with_capacity(num);
		let mut pxtr: *mut Color = pixels.as_mut_ptr();
		let pxend = unsafe { pxtr.offset(num as isize) };
		while pxtr != pxend {
			let mut read = 0;
			{
				let mut data = file.fill_buf()?;
				while data.len() >= 3 {
					unsafe {
						*pxtr = Color {
							blue: data[2],
							green: data[1],
							red: data[0],
							undef: 0,
						};
					}
					read += 3;
					data = &data[3..];
					pxtr = unsafe { pxtr.offset(1) };
				}
			}
			file.consume(read);
		}
		unsafe { pixels.set_len(num); }
		Ok(Image {
			pixels: pixels,
			width: width as i32,
			height: height as i32,
		})
	}
}
impl Default for Image {
	fn default() -> Image {
		Image {
			pixels: Vec::new(),
			width: 0,
			height: 0,
		}
	}
}
impl AsRef<[Color]> for Image {
	fn as_ref(&self) -> &[Color] {
		&self.pixels
	}
}
impl AsMut<[Color]> for Image {
	fn as_mut(&mut self) -> &mut [Color] {
		&mut self.pixels
	}
}
