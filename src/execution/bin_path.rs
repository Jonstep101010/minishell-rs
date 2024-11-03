use ::libc;
extern "C" {
	fn get_env(env: *const *mut libc::c_char, key: *const libc::c_char) -> *mut libc::c_char;
	fn free(_: *mut libc::c_void);
	fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
	fn ft_strchr(str: *const libc::c_char, c: libc::c_int) -> *mut libc::c_char;
	fn ft_strdup(s: *const libc::c_char) -> *mut libc::c_char;
	fn ft_strjoin(s1: *const libc::c_char, s2: *const libc::c_char) -> *mut libc::c_char;
	fn ft_split(s: *const libc::c_char, c: libc::c_char) -> *mut *mut libc::c_char;
	fn arr_free(arr: *mut *mut libc::c_char);
	fn free_null(p: *mut libc::c_void);
	fn eprint(fmt: *const libc::c_char, _: ...);
}
pub type uint8_t = __uint8_t;
pub type __uint8_t = libc::c_uchar;
unsafe extern "C" fn find_bin(
	mut paths: *mut *const libc::c_char,
	mut binprefix: *const libc::c_char,
	mut binpath_buf: *mut *mut libc::c_char,
) -> uint8_t {
	if binprefix.is_null() {
		return 1 as libc::c_int as uint8_t;
	}
	while !paths.is_null() && !(*paths).is_null() {
		*binpath_buf = ft_strjoin(*paths, binprefix);
		if !binpath_buf.is_null() && (*binpath_buf).is_null() {
			return 1 as libc::c_int as uint8_t;
		}
		if access(*binpath_buf, 0 as libc::c_int) == 0 as libc::c_int {
			return 0 as libc::c_int as uint8_t;
		}
		free_null(binpath_buf as *mut libc::c_void);
		paths = paths.offset(1);
	}
	return 127 as libc::c_int as uint8_t;
}
unsafe extern "C" fn get_bin(
	mut paths: *mut *const libc::c_char,
	mut bin: *const libc::c_char,
	mut binpath_buf: *mut *mut libc::c_char,
) -> uint8_t {
	let mut bin_prefix: *const libc::c_char =
		ft_strjoin(b"/\0" as *const u8 as *const libc::c_char, bin);
	let mut status: uint8_t = 0;
	if bin_prefix.is_null() {
		arr_free(paths as *mut *mut libc::c_char);
		return 1 as libc::c_int as uint8_t;
	}
	if *bin == 0 {
		free(bin_prefix as *mut libc::c_char as *mut libc::c_void);
		*binpath_buf = ft_strdup(b"\0" as *const u8 as *const libc::c_char);
		arr_free(paths as *mut *mut libc::c_char);
		return 127 as libc::c_int as uint8_t;
	}
	status = find_bin(paths, bin_prefix, binpath_buf);
	arr_free(paths as *mut *mut libc::c_char);
	free(bin_prefix as *mut libc::c_char as *mut libc::c_void);
	if status as libc::c_int == 0 as libc::c_int
		&& access(*binpath_buf, 1 as libc::c_int) == -(1 as libc::c_int)
	{
		return 126 as libc::c_int as uint8_t;
	}
	return status;
}
#[no_mangle]
pub unsafe extern "C" fn set_single(
	mut env: *const *mut libc::c_char,
	mut bin: libc::c_char,
	mut binpath_buf: *mut *mut libc::c_char,
) -> uint8_t {
	if bin as libc::c_int == '/' as i32 {
		eprint(b"/: Is a directory\0" as *const u8 as *const libc::c_char);
		return 126 as libc::c_int as uint8_t;
	}
	if bin as libc::c_int == '.' as i32 {
		eprint(
			b".: filename argument required\0" as *const u8 as *const libc::c_char,
			*binpath_buf,
		);
		return 2 as libc::c_int as uint8_t;
	}
	*binpath_buf = get_env(env, b"HOME\0" as *const u8 as *const libc::c_char);
	if (*binpath_buf).is_null() {
		eprint(b"/home/minishell: Is a directory\0" as *const u8 as *const libc::c_char);
	} else {
		eprint(
			b"%s: Is a directory\0" as *const u8 as *const libc::c_char,
			*binpath_buf,
		);
	}
	return 126 as libc::c_int as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn set_binpath(
	mut env: *const *mut libc::c_char,
	mut bin: *const libc::c_char,
	mut binpath_buf: *mut *mut libc::c_char,
) -> uint8_t {
	let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
	let mut paths: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
	if bin.is_null() {
		return 1 as libc::c_int as uint8_t;
	}
	if *bin as libc::c_int != 0
		&& !(ft_strchr(
			b"~/.\0" as *const u8 as *const libc::c_char,
			*bin as libc::c_int,
		))
		.is_null()
		&& *bin.offset(1 as libc::c_int as isize) == 0
	{
		return set_single(env, *bin, binpath_buf);
	}
	if *bin as libc::c_int == '.' as i32 || *bin as libc::c_int == '/' as i32 {
		*binpath_buf = ft_strdup(bin);
		if *bin as libc::c_int == '.' as i32
			&& *bin.offset(1 as libc::c_int as isize) as libc::c_int == '.' as i32
			&& *bin.offset(2 as libc::c_int as isize) == 0
		{
			return 127 as libc::c_int as uint8_t;
		}
		if (*binpath_buf).is_null() {
			return 1 as libc::c_int as uint8_t;
		}
		if access(*binpath_buf, 0 as libc::c_int) == -(1 as libc::c_int) {
			return 127 as libc::c_int as uint8_t;
		}
		if access(*binpath_buf, 1 as libc::c_int) == -(1 as libc::c_int) {
			return 126 as libc::c_int as uint8_t;
		}
		return 0 as libc::c_int as uint8_t;
	}
	path = get_env(env, b"PATH\0" as *const u8 as *const libc::c_char);
	paths = ft_split(path, ':' as i32 as libc::c_char) as *mut *const libc::c_char;
	free(path as *mut libc::c_void);
	return get_bin(paths, bin, binpath_buf);
}
