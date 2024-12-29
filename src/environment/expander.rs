use std::ffi::{CStr, CString};

use ::libc;
use libc::free;

use libft_rs::ft_substr::ft_substr;

use crate::size_t;

use super::Env;

fn check_index_advance(bytes_s: &[u8], mut i: usize) -> usize {
	let mut count: usize = 0;
	const CHARMATCH: &[u8; 9] = b"$\"'/? )(\0";
	while bytes_s[i + count] != 0
		&& bytes_s[i + count + 1] != 0
		&& CHARMATCH
			.iter()
			.position(|&x| x == bytes_s[i + count + 1])
			.is_none()
	{
		count += 1;
	}
	if bytes_s[i + count] != 0 && bytes_s[i + count + 1] == b'?' {
		count += 1;
	}
	count
}
unsafe fn expand_inside(mut key: *mut libc::c_char, _env: &Env, mut i: &mut usize) -> String {
	let key_c_str = CStr::from_ptr(key);
	let mut ret: String = {
		if *key != 0
			&& let Some(expansion) = _env.get(key_c_str.to_str().unwrap())
		{
			expansion.to_string()
		} else {
			String::new()
		}
	};
	free(key as *mut libc::c_void);
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
unsafe fn expand(s: &CStr, env: &Env) -> Option<CString> {
	let mut i: usize = 0;
	let mut expand_0: bool = false;
	let mut double_quote = 0;
	// let mut s = String::from(s.to_str().unwrap());
	// let mut ret = CString::new("");
	let mut ret = String::new();
	loop {
		let bytes_s = s.to_bytes_with_nul();
		if bytes_s[0] == b'\0' {
			break;
		}
		// 		check_quotes(&*s.offset(i as isize), &mut expand_0, &mut double_quote);
		check_quotes(&bytes_s[i..], &mut expand_0, &mut double_quote);
		const EXP_CHARS: &[u8; 4] = b"$()\0";
		if expand_0 != false
			&& bytes_s[i] == b'$'
			&& bytes_s[i + 1] != 0
			&& EXP_CHARS
				.iter()
				.position(|&x| x == bytes_s[i + 1])
				.is_none()
		{
			let mut key: *mut libc::c_char = ft_substr(
				s.as_ptr(),
				(i + 1) as libc::c_uint,
				check_index_advance(s.to_bytes_with_nul(), i) as size_t,
			);
			if key.is_null() {
				// panic!("alloc fail");
				return None;
			}
			let expansion = expand_inside(key, env, &mut i);
			ret.push_str(&expansion);
		} else {
			ret.push(char::from(s.to_bytes()[i]));
		}
		i += 1;
	}
	Some(CString::new(ret).unwrap())
}
pub fn expander(input_expander: &CStr, env: &Env) -> Option<CString> {
	if input_expander.is_empty() {
		return None;
	}
	unsafe { expand(input_expander, env) }
}
