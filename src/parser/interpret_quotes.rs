use std::ffi::CString;

use libc::{c_char, c_int, c_ulong};
use libft_rs::{ft_calloc::ft_calloc, ft_strlen::ft_strlen};

///
/// # Safety
///
/// assume s is not null and is nul-terminated
pub unsafe fn do_quote_bs(mut s: *const c_char, quote: &mut c_int) -> *mut c_char {
	let tmp = ft_calloc(
		ft_strlen(s) + 1_u64,
		::core::mem::size_of::<c_char>() as c_ulong,
	) as *mut c_char;
	if tmp.is_null() {
		return std::ptr::null_mut::<c_char>();
	}
	while *s != 0 {
		match (*quote as u8, *s as u8) {
			(0, b'\'' | b'"') => *quote = *s as c_int,
			(_, q) if q == *quote as u8 => {
				*quote = 0;
			}
			_ => {
				let tmp_len = ft_strlen(tmp) as usize;
				*tmp.add(tmp_len) = *s;
				*tmp.add(tmp_len + 1) = '\0' as c_char;
			}
		}
		s = s.add(1);
	}
	tmp
}

pub fn rs_do_quote_bs(bytes_s: &[u8], quote: &mut c_int) -> CString {
	let mut tmp = Vec::new();
	for &byte in bytes_s {
		match (*quote as u8, byte) {
			(0, b'\'' | b'"') => *quote = byte as c_int,
			(_, q) if q == *quote as u8 => {
				*quote = 0;
			}
			_ => {
				tmp.push(byte);
			}
		}
	}
	unsafe { CString::from_vec_unchecked(tmp) }
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
		unsafe {
			#[allow(deprecated)]
			let output = super::do_quote_bs(input.as_ptr(), &mut ptr_int_quote);
			assert_eq!(expected, std::ffi::CStr::from_ptr(output).to_str().unwrap());
			libc::free(output.cast());
			let output = super::rs_do_quote_bs(input.as_bytes_with_nul(), &mut ptr_int_quote);
			assert_eq!(expected, output.to_str().unwrap());
		};
	}
}
