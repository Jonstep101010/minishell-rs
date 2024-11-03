use ::libc;
extern "C" {
	fn ft_strlen(str: *const libc::c_char) -> size_t;
	fn ft_substr(s: *const libc::c_char, start: libc::c_uint, len: size_t) -> *mut libc::c_char;
	fn get_index_env(env: *const *mut libc::c_char, substr: *const libc::c_char) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn get_env(
	mut env: *const *mut libc::c_char,
	mut key: *const libc::c_char,
) -> *mut libc::c_char {
	let mut index: libc::c_int = 0;
	if env.is_null() || key.is_null() {
		return std::ptr::null_mut::<libc::c_char>();
	}
	index = get_index_env(env, key);
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
