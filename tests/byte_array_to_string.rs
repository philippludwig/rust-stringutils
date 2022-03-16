extern crate stringutils;

#[cfg(test)]
mod tests {
    use stringutils::byte_array_to_string;

    #[test]
    fn test_ba_to_string() {
        let byte_array = vec!(0x0au8, 0x20u8, 0x4Fu8);
        let repr = byte_array_to_string(&byte_array);
        assert_eq!(repr, String::from("0a204f"));
        println!("{}", repr);
    }
}
