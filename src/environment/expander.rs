#![warn(clippy::pedantic)]

use super::Env;
use std::ffi::{CStr, CString};

const CHARMATCH: &[u8; 9] = b"$\"'/? )(\0";

fn check_index_advance(bytes_s_at_i: &[u8]) -> usize {
	let mut count: usize = 0;
	while !CHARMATCH.iter().any(|&x| x == bytes_s_at_i[count + 1]) {
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
	mut key: &CStr,
	mut env: &Env,
	mut i: *mut libc::c_int,
) -> *mut libc::c_char {
	*i = (*i as libc::c_ulong).wrapping_add(key.count_bytes() as u64) as libc::c_int as libc::c_int;
	if !key.is_empty() {
		if let Some(val) = env.get(key.to_str().unwrap()) {
			let cstr = CString::new(val.as_str()).expect("valid cstring from val");
			libft_rs::ft_strdup(cstr.as_ptr())
		} else {
			libft_rs::ft_strdup(b"\0" as *const u8 as *const libc::c_char)
		}
	} else {
		libft_rs::ft_strdup(c"$".as_ptr())
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
	let mut should_expand: bool = true;
	let mut double_quote = false;
	let mut ret = String::new();
	let bytes = input_expander.to_bytes_with_nul();
	while bytes[i] != b'\0' {
		match bytes[i] {
			b'"' => {
				double_quote = !double_quote;
			}
			b'\'' if !double_quote && bytes[check_index_advance(&bytes[i..])] != b'"' => {
				should_expand = !should_expand;
			}
			_ => {}
		}
		if should_expand && bytes[i] == b'$' && !EXP_CHARS.iter().any(|&x| x == bytes[i + 1]) {
			let key_slice = &bytes[i + 1..check_index_advance(&bytes[i..]) + i + 1];
			dbg!(&key_slice);
			let key_slice = CString::new(key_slice).unwrap();
			let expansion = expand_inside(&key_slice, env, &mut i);
			let expansion_c = expand_inside_c(&key_slice, env, &mut (i as i32));
			assert_eq!(
				*CStr::from_ptr(expansion_c),
				*CString::new(expansion.clone()).unwrap().as_c_str()
			);
			libc::free(expansion_c.cast());
			ret.push_str(&expansion);
		} else {
			ret.push(bytes[i].into());
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
