use ::libc;
use libft_rs::{ft_strlen::ft_strlen, ft_substr::ft_substr};

use super::get_index::get_index_env;

#[no_mangle]
pub unsafe extern "C" fn get_env(
	mut env: *const *mut libc::c_char,
	mut key: *const libc::c_char,
) -> *mut libc::c_char {
	if env.is_null() || key.is_null() {
		return std::ptr::null_mut::<libc::c_char>();
	}
	let mut index: libc::c_int = get_index_env(env, key);
	if index != -(1 as libc::c_int) && !(*env.offset(index as isize)).is_null() {
		return ft_substr(
			*env.offset(index as isize),
			(ft_strlen(key)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_uint,
			(ft_strlen(*env.offset(index as isize)))
				.wrapping_sub(ft_strlen(key))
				.wrapping_add(1 as libc::c_int as libc::c_ulong),
		);
	}
	std::ptr::null_mut::<libc::c_char>()
}
