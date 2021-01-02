use std::fmt::Write;

extern "C" {
	fn strftime(s: *mut libc::c_char, max: libc::size_t, format: *const libc::c_char, tm: *const libc::tm)
		-> usize;
}

/// Convert a byte array to a ``String``.
///
/// # Examples
///
/// ```
/// let byte_array: Vec<u8> = vec!(0x0F, 0x5F, 0xAA);
/// let repr: String = stringutils::byte_array_to_string(&byte_array);
///
/// println!("Bytes: {}", repr);
/// ```
pub fn byte_array_to_string(bytes: &[u8]) -> String {
	let mut repr = String::new();
	for x in bytes {
		write!(&mut repr, "{:02x}", x).unwrap();
	}
	repr
}

/// Get a String representation for a timestamp.
///
/// # Example
/// ```
/// let current_time = unsafe { libc::time(std::ptr::null_mut()) };
/// let repr = stringutils::timestamp_to_string(current_time).unwrap();
/// println!("Current time: {}", repr);
/// ```
pub fn timestamp_to_string(timestamp_secs: i64) -> Result<String, std::str::Utf8Error> {
	let gmtime = unsafe { libc::gmtime(&timestamp_secs) };
	let mut buf = [0u8; 4096];

	let format_str = "%Y-%m-%d %H:%M";
	let format_cstr = std::ffi::CString::new(format_str).unwrap();

	let len = unsafe { strftime(buf.as_mut_ptr() as *mut i8, 4096, format_cstr.as_ptr(), gmtime) };

	std::str::from_utf8(&buf[0..len]).map(|s| s.to_owned())
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_ba_to_string() {
		let byte_array = vec!(0x0au8, 0x20u8, 0x4Fu8);
		let repr = super::byte_array_to_string(&byte_array);
		assert_eq!(repr, String::from("0a204f"));
		println!("{}", repr);
	}
}

