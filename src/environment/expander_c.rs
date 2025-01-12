use libc::{self, c_char};
use std::{
	ffi::{CStr, CString},
	ptr::null_mut,
};

use super::Env;

unsafe extern "C" {
	fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
	fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
	fn strlen(_: *const libc::c_char) -> libc::c_ulong;
	fn free(_: *mut libc::c_void);
	fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
	fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
unsafe fn check_index_advance(mut s: *const libc::c_char, mut i: libc::c_int) -> libc::c_int {
	let mut count: libc::c_int = 0 as libc::c_int;
	while *s.offset((i + count) as isize) as libc::c_int != 0
		&& *s.offset((i + count + 1 as libc::c_int) as isize) as libc::c_int != 0
		&& (strchr(
			b"$\"'/? )(\0" as *const u8 as *const libc::c_char,
			*s.offset((i + count + 1 as libc::c_int) as isize) as libc::c_int,
		))
		.is_null()
	{
		count += 1;
		count;
	}
	if *s.offset((i + count) as isize) as libc::c_int != 0
		&& *s.offset((i + count + 1 as libc::c_int) as isize) as libc::c_int == '?' as i32
	{
		count += 1 as libc::c_int;
	}
	return count;
}
unsafe fn expand_inside(
	mut key: *mut libc::c_char,
	mut env: &Env,
	mut i: *mut libc::c_int,
) -> *mut libc::c_char {
	let len: size_t = strlen(key);
	let mut ret = std::ptr::null_mut::<c_char>();
	let cstr = CStr::from_ptr(key);
	if *key != 0 {
		if let Some(val) = env.get(cstr.to_str().unwrap()) {
			let cstr = CString::new(val.as_str()).expect("valid cstring from val");
			ret = strdup(cstr.as_ptr());
		}
	} else {
		ret = strdup(c"$".as_ptr());
	}
	if ret.is_null() {
		ret = strdup(b"\0" as *const u8 as *const libc::c_char);
	}
	free(key as *mut libc::c_void);
	if ret.is_null() {
		return 0 as *mut libc::c_char;
	}
	*i = (*i as libc::c_ulong).wrapping_add(len) as libc::c_int as libc::c_int;
	return ret;
}
unsafe fn check_quotes(
	mut s: *const libc::c_char,
	mut expand_0: *mut bool,
	mut double_quote: *mut libc::c_int,
) {
	if *s as libc::c_int == '"' as i32 && *double_quote == 0 as libc::c_int {
		*double_quote = 1 as libc::c_int;
	} else if *s as libc::c_int == '"' as i32 && *double_quote == 1 as libc::c_int {
		*double_quote = 0 as libc::c_int;
	}
	if *s as libc::c_int == '\'' as i32
		&& *double_quote == 0 as libc::c_int
		&& *s.offset(check_index_advance(s, 0 as libc::c_int) as isize) as libc::c_int != '"' as i32
	{
		*expand_0 = !*expand_0;
	}
}

pub unsafe fn ft_strjoin(
	mut s1: *const libc::c_char,
	mut s2: *const libc::c_char,
) -> *mut libc::c_char {
	let mut s1_len: size_t = strlen(s1);
	let mut s2_len: size_t = strlen(s2);
	let mut joined: *mut libc::c_char = malloc(
		s1_len
			.wrapping_add(s2_len)
			.wrapping_add(1 as libc::c_int as libc::c_ulong),
	) as *mut libc::c_char;
	if joined.is_null() {
		return 0 as *mut libc::c_char;
	}
	libft_rs::ft_strlcpy(
		joined,
		s1,
		s1_len.wrapping_add(1 as libc::c_int as libc::c_ulong),
	);
	libft_rs::ft_strlcat(
		joined,
		s2,
		s1_len
			.wrapping_add(s2_len)
			.wrapping_add(1 as libc::c_int as libc::c_ulong),
	);
	return joined;
}
pub unsafe fn free_both_join(
	mut s1: *mut libc::c_char,
	mut s2: *mut libc::c_char,
) -> *mut libc::c_char {
	let mut joined: *mut libc::c_char = 0 as *mut libc::c_char;
	joined = ft_strjoin(s1 as *const libc::c_char, s2 as *const libc::c_char);
	free(s1 as *mut libc::c_void);
	free(s2 as *mut libc::c_void);
	return joined;
}
pub unsafe fn append_char_str(mut s: *mut libc::c_char, mut c: libc::c_char) -> *mut libc::c_char {
	let len: size_t = (strlen(s)).wrapping_add(2 as libc::c_int as libc::c_ulong);
	let str: [libc::c_char; 2] = [c, '\0' as i32 as libc::c_char];
	if s.is_null() {
		return strdup(str.as_ptr());
	}
	let mut res: *mut libc::c_char =
		calloc(len, ::core::mem::size_of::<libc::c_char>() as libc::c_ulong) as *mut libc::c_char;
	if res.is_null() {
		free(s as *mut libc::c_void);
		return 0 as *mut libc::c_void as *mut libc::c_char;
	}
	libft_rs::ft_strlcpy(res, s, len.wrapping_sub(1 as libc::c_int as libc::c_ulong));
	libft_rs::ft_strlcat(res, str.as_ptr(), len);
	free(s as *mut libc::c_void);
	return res;
}
unsafe fn expand(s: *const libc::c_char, mut env: &Env) -> Option<CString> {
	let mut i: libc::c_int = -(1 as libc::c_int);
	let mut expand_0: bool = 1 as libc::c_int != 0;
	let mut double_quote: libc::c_int = 0 as libc::c_int;
	let mut ret: *mut libc::c_char = strdup(b"\0" as *const u8 as *const libc::c_char);
	let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
	loop {
		i += 1;
		if !(*s.offset(i as isize) != 0) {
			break;
		}
		check_quotes(&*s.offset(i as isize), &mut expand_0, &mut double_quote);
		if expand_0 as libc::c_int != 0
			&& (*s.offset(i as isize) as libc::c_int == '$' as i32
				&& *s.offset((i + 1 as libc::c_int) as isize) as libc::c_int != 0
				&& (strchr(
					b"$()\0" as *const u8 as *const libc::c_char,
					*s.offset((i + 1 as libc::c_int) as isize) as libc::c_int,
				))
				.is_null())
		{
			key = libft_rs::ft_substr(s, i as u32 + 1, check_index_advance(s, i) as size_t)
				as *mut libc::c_char;
			if key.is_null() {
				free(ret as *mut libc::c_void);
				return None;
			}
			ret = free_both_join(ret, expand_inside(key, env, &mut i));
		} else {
			ret = append_char_str(ret, *s.offset(i as isize));
		}
	}
	Some(CString::from(CStr::from_ptr(ret)))
}

/// Expand the input string using the environment variables stored in the `env` struct.
///
/// # Arguments
/// `input_expander` - A `CStr` reference to the input string to be expanded.
#[must_use]
pub unsafe fn expander(input_expander: &CStr, env: &Env) -> Option<CString> {
	if input_expander.is_empty() {
		return None;
	}
	if (strchr(input_expander.as_ptr(), '$' as i32)).is_null() {
		return Some(input_expander.to_owned());
	}
	expand(input_expander.as_ptr(), env)
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::ffi::CString;

	//$'USER', $"USER" should not expand
	use rstest::{fixture, rstest};

	// use $USER env var only
	// expected, input for expander
	#[rstest]
	#[case("'$USER'", "'$USER'")]
	#[case(&std::env::var("USER").unwrap(), "$USER")]
	#[case(&format!("echo \"'{}'\"", std::env::var("USER").unwrap()), "echo \"'$USER'\"")]
	#[case("echo '\"$USER\"'", "echo '\"$USER\"'")]
	#[case(&format!("Hello {}", std::env::var("USER").unwrap()), "Hello $USER")]
	#[case(&format!("Hello ${}", std::env::var("USER").unwrap()), "Hello $$USER")]
	#[case("", "$USERsomething")]
	#[case("echo \"''\"", "echo \"'$USERsomething'\"")]
	#[case("echo \"'\"'$PAGER'\"'\"", "echo \"'\"'$PAGER'\"'\"")]
	#[case(&format!("echo {}{}", std::env::var("USER").unwrap(), std::env::var("USER").unwrap()), "echo $USER$USER")]
	#[case(&format!("echo \"{}\"{}", std::env::var("USER").unwrap(), std::env::var("USER").unwrap()), "echo \"$USER\"$USER")]
	#[case(&format!("echo \"{} something\"", std::env::var("USER").unwrap()), "echo \"$USER something\"")]
	#[case(&format!("echo {} something", std::env::var("USER").unwrap()), "echo $USER something")]
	#[case(&format!("echo {}", std::env::var("USER").unwrap()), "echo $USER$something")]
	#[case("echo something strange", "echo something strange")]
	#[case(&	format!(
		"echo \"{}\"{}",
		std::env::var("USER").unwrap(),
		std::env::var("USER").unwrap(),
	),"echo \"$USER\"$USER")]
	#[case(&format!(
				"echo {} | echo \"{}\"",
				std::env::var("USER").unwrap(),
				std::env::var("USER").unwrap()
	),"echo $USER | echo \"$USER\"")]
	#[fixture]
	fn test_expander(#[case] expected: &str, #[case] input: &str) {
		let env = Env::new();
		unsafe {
			let input = CString::new(input).unwrap();
			let output = expander(&input, &env).unwrap();
			assert_eq!(expected, output.to_str().unwrap());
		}
	}
	#[rstest]
	// #[case("echo 0", "echo $?")]
	// #[case("0", "$?")]
	#[case("$\"USER\"", "$\"USER\"")]
	#[case("$'USER'", "$'USER'")]
	#[case("echo $'TEST $TEST'", "echo $'TEST $TEST'")]
	#[case("echo $\"42$\"", "echo $\"42$\"")]
	#[case("echo \"$ \"", "echo \"$ \"")]
	#[case(&format!("echo ${}$", std::env::var("USER").unwrap()), "echo $USER$")]
	#[case("echo something $$ strange", "echo something $$ strange")]
	#[fixture]
	fn test_expander_failing(#[case] expected: &str, #[case] input: &str) {
		let env = Env::new();
		unsafe {
			let input = CString::new(input).unwrap();
			let output = expander(&input, &env).unwrap();
			assert_eq!(expected, output.to_str().unwrap());
		}
	}
}
