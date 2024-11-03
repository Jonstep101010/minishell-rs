use ::libc;
extern "C" {
	fn ft_strlen(str: *const libc::c_char) -> size_t;
	fn ft_strncmp(s1: *const libc::c_char, s2: *const libc::c_char, n: size_t) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
unsafe extern "C" fn get_key_len(mut s: *const libc::c_char) -> size_t {
	let mut len: size_t = 0;
	let mut key_len: size_t = 0;
	key_len = 0 as libc::c_int as size_t;
	len = ft_strlen(s);
	if *s as libc::c_int == '=' as i32 {
		return -(1 as libc::c_int) as size_t;
	}
	while key_len < len && *s.offset(key_len as isize) as libc::c_int != '=' as i32 {
		key_len = key_len.wrapping_add(1);
	}
	if *s.offset(key_len as isize) as libc::c_int == '=' as i32
		&& key_len != 0 as libc::c_int as libc::c_ulong
	{
		return key_len;
	}
	if key_len == len {
		return key_len;
	}
	-(1 as libc::c_int) as size_t
}
#[no_mangle]
pub unsafe extern "C" fn get_index_env(
	mut env: *const *mut libc::c_char,
	mut substr: *const libc::c_char,
) -> libc::c_int {
	let mut i: size_t = 0;
	let mut key_len: size_t = 0;
	if env.is_null() || substr.is_null() {
		return -(1 as libc::c_int);
	}
	i = 0 as libc::c_int as size_t;
	key_len = get_key_len(substr);
	if key_len <= 0 as libc::c_int as libc::c_ulong {
		return -(1 as libc::c_int);
	}
	while !(*env.offset(i as isize)).is_null() {
		if ft_strncmp(*env.offset(i as isize), substr, key_len) == 0 as libc::c_int
			&& *(*env.offset(i as isize)).offset(key_len as isize) as libc::c_int != 0
			&& *(*env.offset(i as isize)).offset(key_len as isize) as libc::c_int == '=' as i32
		{
			return i as libc::c_int;
		}
		i = i.wrapping_add(1);
	}
	-(1 as libc::c_int)
}
