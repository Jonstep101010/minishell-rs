#![warn(clippy::pedantic)]
use libc::size_t;

use super::Env;
use std::ffi::{CStr, CString};

const CHARMATCH: &[u8; 8] = b"$\"'/? )(";

fn check_index_advance(bytes_s_at_i: &[u8]) -> usize {
	let mut count: usize = 0;
	while bytes_s_at_i[count] != b'\0'
		&& bytes_s_at_i[count + 1] != b'\0'
		&& !CHARMATCH.iter().any(|&x| x == bytes_s_at_i[count + 1])
	{
		count += 1;
	}
	if bytes_s_at_i[count] != b'\0' && bytes_s_at_i[count + 1] == b'?' {
		count += 1;
	}
	count
}

fn expand_inside(key_c_str: &CStr, env: &Env, mut i: &mut usize) -> String {
	// advance by key length in source string
	*i += key_c_str.count_bytes();
	if key_c_str.is_empty() {
		"$".to_string()
	} else if let Some(expansion) = env.get(key_c_str.to_str().unwrap()) {
		expansion.to_string()
	} else {
		String::from("")
	}
}

unsafe fn expand_inside_c(
	mut key: *mut libc::c_char,
	mut env: &Env,
	mut i: *mut libc::c_int,
) -> *mut libc::c_char {
	let len: size_t = libc::strlen(key);
	let mut ret = std::ptr::null_mut::<libc::c_char>();
	let cstr = CStr::from_ptr(key);
	if *key != 0 {
		if let Some(val) = env.get(cstr.to_str().unwrap()) {
			let cstr = CString::new(val.as_str()).expect("valid cstring from val");
			ret = libft_rs::ft_strdup(cstr.as_ptr());
		}
	} else {
		ret = libft_rs::ft_strdup(c"$".as_ptr());
	}
	if ret.is_null() {
		ret = libft_rs::ft_strdup(b"\0" as *const u8 as *const libc::c_char);
	}
	*i = (*i as libc::c_ulong).wrapping_add(len as u64) as libc::c_int as libc::c_int;
	return ret;
}

fn check_quotes(c: &[u8], mut expand_0: &mut bool, mut double_quote: &mut i32) {
	if c[0] == b'"' && *double_quote == 0 {
		*double_quote = 1;
	} else if c[0] == b'"' && *double_quote == 1 {
		*double_quote = 0;
	}
	if c[0] == b'\'' && *double_quote == 0 && c[check_index_advance(&c[0..])] != b'"' {
		*expand_0 = !*expand_0;
	}
}
const EXP_CHARS: &[u8; 3] = b"$()";

/// Expand the input string using the environment variables stored in the `env` struct.
///
/// # Arguments
/// `input_expander` - A `CStr` reference to the input string to be expanded.
#[must_use]
pub unsafe fn expander(input_expander: &CStr, env: &Env) -> Option<CString> {
	if input_expander.is_empty() {
		return None;
	}
	let mut i = 0;
	let mut expand_0: bool = true;
	let mut double_quote = 0;
	let mut ret = String::new();
	let bytes_s = input_expander.as_ptr();
	let b_s = input_expander.to_bytes_with_nul();
	while b_s[i] != b'\0' {
		check_quotes(&b_s[i..], &mut expand_0, &mut double_quote);
		if expand_0 && b_s[i] == b'$' && !EXP_CHARS.iter().any(|&x| x == b_s[i + 1]) {
			let mut key =
				libft_rs::ft_substr(bytes_s, i as u32 + 1, check_index_advance(&b_s[i..]) as u64)
					as *mut libc::c_char;
			if key.is_null() {
				return None;
			}
			let expansion = expand_inside(CStr::from_ptr(key), env, &mut i);
			let expansion_c = expand_inside_c(key, env, &mut (i as i32));
			assert_eq!(
				*CStr::from_ptr(expansion_c),
				*CString::new(expansion.clone()).unwrap().as_c_str()
			);
			libc::free(expansion_c.cast());
			ret.push_str(&expansion);
		} else {
			ret.push(b_s[i].into());
		}
		i += 1;
	}
	Some(CString::new(ret).unwrap())
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
	#[case(&format!("echo {}$", std::env::var("USER").unwrap()), "echo $USER$")]
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
