#![warn(clippy::pedantic)]

use super::Env;
use std::ffi::CString;

/// Expand the input string using the environment variables stored in the `env` struct.
///
/// # Arguments
/// `input_expander` - A `CStr` reference to the input string to be expanded.
#[allow(clippy::missing_panics_doc, clippy::items_after_statements)]
#[must_use]
pub fn expander(input_expander: &str, env: &Env) -> Option<String> {
	let mut i = 0;
	let mut should_expand = true;
	let mut has_double_quote = false;
	let mut ret = String::new();
	let input_expander = CString::new(input_expander).unwrap();
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
			let key_byte_slice = &bytes[(i + 1)..=(idx_advance(&bytes[i..]) + i)];
			// advance by key length in source string
			i += key_byte_slice.len();
			let expansion = if key_byte_slice.is_empty() {
				"$".to_string()
			} else if let Some(expansion) = env.get_slice(key_byte_slice) {
				expansion.to_string()
			} else {
				String::new()
			};
			ret.push_str(&expansion);
		} else {
			ret.push(bytes[i].into());
		}
		i += 1;
	}
	Some(ret)
}

#[cfg(test)]
mod tests {
	use super::*;

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
		let env = Env::new_test();
		// let input = CString::new(input).unwrap();
		let output = expander(input, &env).unwrap();
		assert_eq!(expected, output);
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
		let env = Env::new_test();
		// let input = CString::new(input).unwrap();
		let output = expander(input, &env).unwrap();
		assert_eq!(expected, output);
	}
	#[rstest]
	#[case("echo 0", "echo $?")]
	#[case("0", "$?")]
	#[case("0$00$$$$", "$?$$?$?$$$$hello?$")]
	#[fixture]
	fn test_expander_status(#[case] expected: &str, #[case] input: &str) {
		let env = Env::new_exit_status();
		// let input = CString::new(input).unwrap();
		let output = expander(input, &env).unwrap();
		assert_eq!(expected, output);
	}
}
