use std::fmt::Write;

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
