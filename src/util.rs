/*!
*/

#[inline]
pub fn from_wchar_buf(buf: &[u16]) -> &[u16] {
	let len = buf.iter()
		.enumerate()
		.find(|&(_, &word)| word == 0)
		.map_or_else(|| buf.len(), |(len, _)| len);
	&buf[..len]
}
