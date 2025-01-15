use ::libc;
use libc::free;
use libft_rs::{ft_calloc::ft_calloc, ft_strlen::ft_strlen};
use libutils_rs::src::string::str_cchr::str_cchr;

use crate::size_t;

pub unsafe fn do_quote_bs(
	mut s: *const libc::c_char,
	mut quote: *mut libc::c_int,
) -> *mut libc::c_char {
	if s.is_null() {
		return std::ptr::null_mut::<libc::c_char>();
	}
	let mut len: size_t = ft_strlen(s);
	let mut tmp: *mut libc::c_char = ft_calloc(
		len.wrapping_add(1 as libc::c_int as libc::c_ulong),
		::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
	) as *mut libc::c_char;
	if tmp.is_null() {
		return std::ptr::null_mut::<libc::c_char>();
	}
	while *s != 0 {
		if *quote == 0 as libc::c_int
			&& (*s as libc::c_int == '\'' as i32 || *s as libc::c_int == '"' as i32)
		{
			*quote = *s as libc::c_int;
		} else if *quote != 0 as libc::c_int && *s as libc::c_int == *quote {
			*quote = 0 as libc::c_int;
		} else {
			let mut tmp_len = ft_strlen(tmp);
			*tmp.offset(tmp_len as isize) = *s;
			*tmp.offset(tmp_len.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize) =
				'\0' as i32 as libc::c_char;
		}
		s = s.offset(1);
	}
	tmp as *mut libc::c_char
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn interpret_quotes(mut cmd_arr: *mut *mut libc::c_char) -> bool {
	let mut i: libc::c_int = 0;
	let mut quote: libc::c_int = 0;
	if cmd_arr.is_null() {
		return 1 as libc::c_int != 0;
	}
	while !(*cmd_arr.offset(i as isize)).is_null() {
		if str_cchr(*cmd_arr.offset(i as isize), '\'' as i32 as libc::c_char) != 0 as libc::c_int
			|| str_cchr(*cmd_arr.offset(i as isize), '"' as i32 as libc::c_char) != 0 as libc::c_int
		{
			let mut tmp: *mut libc::c_char =
				do_quote_bs(*cmd_arr.offset(i as isize), &mut quote) as *mut libc::c_char;
			if tmp.is_null() {
				return 0 as libc::c_int != 0;
			}
			free(*cmd_arr.offset(i as isize) as *mut libc::c_void);
			let fresh0 = &mut (*cmd_arr.offset(i as isize));
			*fresh0 = tmp;
		}
		i += 1;
	}
	1 as libc::c_int != 0
}

#[cfg(test)]
mod tests {
	use rstest::fixture;
	use rstest::rstest;

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
			let output = super::do_quote_bs(input.as_ptr(), &mut ptr_int_quote);
			assert_eq!(expected, std::ffi::CStr::from_ptr(output).to_str().unwrap());
			libc::free(output.cast());
		};
	}
}
