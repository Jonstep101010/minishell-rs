#![warn(clippy::pedantic)]
use super::Env;
use std::ffi::{CStr, CString};

const CHARMATCH: &[u8; 9] = b"$\"'/? )(\0";
fn check_index_advance(bytes_s: &[u8], mut i: usize) -> usize {
	let mut count: usize = 0;
	while bytes_s[i + count] != b'\0'
		&& bytes_s[i + count + 1] != b'\0'
		&& !CHARMATCH.iter().any(|&x| x == bytes_s[i + count + 1])
	{
		count += 1;
	}
	if bytes_s[i + count] != b'\0' && bytes_s[i + count + 1] == b'?' {
		count += 1;
	}
	count
}
fn expand_inside(key_c_str: &CStr, env: &Env, mut i: &mut usize) -> String {
	let mut ret: String = {
		if !key_c_str.is_empty() {
			if let Some(expansion) = env.get(key_c_str.to_str().unwrap()) {
				expansion.to_string()
			} else {
				"$".to_string()
			}
		} else {
			String::from("")
		}
	};
	*i = (*i).wrapping_add(key_c_str.count_bytes());
	ret
}
fn check_quotes(c: &[u8], mut expand_0: &mut bool, mut double_quote: &mut i32) {
	if c[0] == b'"' && *double_quote == 0 {
		*double_quote = 1;
	} else if c[0] == b'"' && *double_quote == 1 {
		*double_quote = 0;
	}
	if c[0] == b'\'' && *double_quote == 0 && c[check_index_advance(c, 0)] != b'"' {
		*expand_0 = !*expand_0;
	}
}
const EXP_CHARS: &[u8; 4] = b"$()\0";
fn expand(s: &CStr, env: &Env) -> CString {
	let mut i: usize = 0;
	let mut expand_0: bool = false;
	let mut double_quote = 0;
	let mut ret = String::new();
	let bytes_s = s.to_bytes_with_nul();
	loop {
		if bytes_s[0] == b'\0' {
			break;
		}
		check_quotes(&bytes_s[i..], &mut expand_0, &mut double_quote);
		if !expand_0
			&& bytes_s[i] == b'$'
			&& bytes_s[i + 1] != b'\0'
			&& !EXP_CHARS.iter().any(|&x| x == bytes_s[i + 1])
		{
			let start = i + 1;
			let len = check_index_advance(bytes_s, i);
			unsafe {
				let key = CStr::from_bytes_with_nul_unchecked(&bytes_s[start..=(start + len)]);
				let expansion = expand_inside(key, env, &mut i);
				ret.push_str(&expansion);
			}
		} else if i < bytes_s.len() - 1 {
			ret.push(bytes_s[i] as char);
		} else {
			break;
		}
		i += 1;
	}
	CString::new(ret).unwrap()
}

/// Expand the input string using the environment variables stored in the `env` struct.
///
/// # Arguments
/// `input_expander` - A `CStr` reference to the input string to be expanded.
#[must_use]
pub fn expander(input_expander: &CStr, env: &Env) -> Option<CString> {
	if input_expander.is_empty() {
		return None;
	}
	Some(expand(input_expander, env))
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
	#[case("echo 0", "echo $?")]
	#[case("0", "$?")]
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
		let input = CString::new(input).unwrap();
		let output = expander(&input, &env).unwrap();
		assert_eq!(expected, output.to_str().unwrap());
	}
}
