// ///
// /// # Safety
// ///
// /// assume s is not null and is nul-terminated
// pub unsafe fn do_quote_bs(mut s: *const libc::c_char, quote: &mut libc::c_int) -> *mut libc::c_char {
// 	let tmp = libft_rs::ft_calloc::ft_calloc(
// 		libft_rs::ft_strlen::ft_strlen(s) + 1_u64,
// 		::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
// 	) as *mut libc::c_char;
// 	if tmp.is_null() {
// 		return std::ptr::null_mut::<libc::c_char>();
// 	}
// 	while *s != 0 {
// 		match (*quote as u8, *s as u8) {
// 			(0, b'\'' | b'"') => *quote = *s as libc::c_int,
// 			(_, q) if q == *quote as u8 => {
// 				*quote = 0;
// 			}
// 			_ => {
// 				let tmp_len = ft_strlen(tmp) as usize;
// 				*tmp.add(tmp_len) = *s;
// 				*tmp.add(tmp_len + 1) = '\0' as libc::c_char;
// 			}
// 		}
// 		s = s.add(1);
// 	}
// 	tmp
// }

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
		// unsafe {
		// #[allow(deprecated)]
		// let output = super::do_quote_bs(input.as_ptr(), &mut ptr_int_quote);
		// assert_eq!(expected, std::ffi::CStr::from_ptr(output).to_str().unwrap());
		// libc::free(output.cast());
		let output = super::rs_do_quote_bs(input.as_bytes_with_nul(), &mut ptr_int_quote);
		assert_eq!(expected, output);
		// };
	}
}
