pub(super) fn rs_do_quote_bs(bytes_s: &[u8], quote: &mut i32) -> String {
	let mut tmp = String::new();
	for &byte in bytes_s {
		match (*quote as u8, byte) {
			(0, b'\'' | b'"') => *quote = byte as i32,
			(_, q) if q == *quote as u8 => {
				*quote = 0;
			}
			_ => {
				tmp.push(byte as char);
			}
		}
	}
	tmp
}

#[cfg(test)]
mod tests {
	use rstest::{fixture, rstest};

	// expected, input
	#[rstest]
	#[case("'\"test\"'", "\"'\"'\"'test'\"'\"'\"")]
	#[case("'test'", "\"'\"test\"'\"")]
	#[case("echo ' \" test ' \"", "echo \"'\" '\"' test \"'\" '\"'")]
	#[case("echo test", "echo ''test''")]
	#[case("echo test", "echo \"\"test\"\"")]
	#[case("'hello'", "\"'hello'\"")]
	#[case("\"hello\"", "'\"hello\"'")]
	#[case(
		"echo hello tehre hello inside single \"jesus\"",
		"echo 'hello tehre 'hello inside single' \"jesus\"'"
	)]
	#[case(
		"echo hello tehre 'hello inside single' jesus",
		"echo \"hello tehre 'hello inside single' \"jesus\"\""
	)]
	#[case(
		"echo hello tehre hello inside single jesus\"\"",
		"echo 'hello tehre 'hello inside single' jesus\"\"'"
	)]
	#[case(
		"echo hello tehre hello inside single jesus",
		"echo 'hello tehre 'hello inside single' jesus'''"
	)]
	#[case(
		"echo hello tehre hello inside single \"\"jesus\"\"",
		"echo 'hello tehre 'hello inside single' \"\"jesus\"\"'"
	)]
	#[fixture]
	fn test_quote_separation(#[case] expected: &str, #[case] input: &str) {
		let input = std::ffi::CString::new(input).unwrap();
		let mut ptr_int_quote = 0;
		let output = super::rs_do_quote_bs(input.as_bytes_with_nul(), &mut ptr_int_quote);
		assert_eq!(expected, output);
	}
}
