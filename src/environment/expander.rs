#![warn(clippy::pedantic)]

use super::Env;
use std::ffi::{CStr, CString};

/// Expand the input string using the environment variables stored in the `env` struct.
///
/// # Arguments
/// `input_expander` - A `CStr` reference to the input string to be expanded.
#[must_use]
pub fn expander(input_expander: &CStr, env: &Env) -> Option<CString> {
	let mut i = 0;
	let mut should_expand = true;
	let mut has_double_quote = false;
	let mut ret = String::new();
	let bytes = input_expander.to_bytes_with_nul();
	const CHARMATCH: &[u8; 9] = b"$\"'/? )(\0";
	let idx_advance = |bytes_at_i: &[u8]| {
		let mut count: usize = 0;
		while !CHARMATCH.iter().any(|&x| x == bytes_at_i[count + 1]) {
			count += 1;
		}
		if bytes_at_i[count] != b'\0' && bytes_at_i[count + 1] == b'?' {
			count += 1;
		}
		count
	};
	while i < input_expander.count_bytes() {
		if bytes[i] == b'"' {
			has_double_quote = !has_double_quote;
		} else if bytes[i] == b'\'' && !has_double_quote && bytes[idx_advance(&bytes[i..])] != b'"'
		{
			should_expand = !should_expand;
		}
		if bytes[i] == b'$' && should_expand && !b"$()".contains(&bytes[i + 1]) {
			let key_byte_slice = &bytes[i + 1..idx_advance(&bytes[i..]) + i + 1];
			// advance by key length in source string
			i += key_byte_slice.len();
			let expansion = if key_byte_slice.len() == 0 {
				"$".to_string()
			} else if let Some(&ref expansion) = env.get_slice(key_byte_slice) {
				expansion.to_string()
			} else {
				String::from("")
			};
			#[cfg(test)]
			{
				unsafe {
					let expansion_c = if !key_byte_slice.is_empty() {
						if let Some(val) = env.get_slice(key_byte_slice) {
							let cstr = CString::new(val.as_str()).expect("valid cstring from val");
							libft_rs::ft_strdup(cstr.as_ptr())
						} else {
							libft_rs::ft_strdup(b"\0" as *const u8 as *const libc::c_char)
						}
					} else {
						libft_rs::ft_strdup(c"$".as_ptr())
					};
					assert_eq!(
						*CStr::from_ptr(expansion_c),
						*CString::new(expansion.clone()).unwrap().as_c_str()
					);
					libc::free(expansion_c.cast());
				}
			}
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
		let input = CString::new(input).unwrap();
		let output = expander(&input, &env).unwrap();
		assert_eq!(expected, output.to_str().unwrap());
	}
	#[rstest]
	#[case("$\"USER\"", "$\"USER\"")]
	#[case("$'USER'", "$'USER'")]
	#[case("echo $'TEST $TEST'", "echo $'TEST $TEST'")]
	#[case("echo $\"42$\"", "echo $\"42$\"")]
	#[case("echo \"$ \"", "echo \"$ \"")]
	#[case(&format!("echo {}$", std::env::var("USER").unwrap()), "echo $USER$")]
	#[case("echo something $$ strange", "echo something $$ strange")]
	#[fixture]
	fn test_expander_weird(#[case] expected: &str, #[case] input: &str) {
		let env = Env::new();
		let input = CString::new(input).unwrap();
		let output = expander(&input, &env).unwrap();
		assert_eq!(expected, output.to_str().unwrap());
	}
	#[rstest]
	#[case("echo 0", "echo $?")]
	#[case("0", "$?")]
	#[case("0$00$$$$", "$?$$?$?$$$$hello?$")]
	#[fixture]
	fn test_expander_status(#[case] expected: &str, #[case] input: &str) {
		let env = Env::new_exit_status();
		let input = CString::new(input).unwrap();
		let output = expander(&input, &env).unwrap();
		assert_eq!(expected, output.to_str().unwrap());
	}
}
