/*!
*/

#![allow(unused)]

#[inline]
pub fn from_wchar_buf(buf: &[u16]) -> &[u16] {
	let len = buf.iter()
		.enumerate()
		.find(|&(_, &word)| word == 0)
		.map_or_else(|| buf.len(), |(len, _)| len);
	&buf[..len]
}

#[inline]
pub fn from_char_buf(buf: &[u8]) -> &[u8] {
	let mut len = buf.len();
	for i in 0..len {
		if buf[i] == 0 {
			len = i;
			break;
		}
	}
	&buf[..len]
}
