use ::libc;
use libc::{access, free};

use libft_rs::{
	ft_split::ft_split, ft_strchr::ft_strchr, ft_strdup::ft_strdup, ft_strjoin::ft_strjoin,
};
use libutils_rs::src::{array::arr_free::arr_free, utils::free_mem::free_null};

use crate::{environment::get_env::get_env, utils::error::eprint};
unsafe extern "C" fn find_bin(
	mut paths: *mut *const libc::c_char,
	mut binprefix: *const libc::c_char,
	mut binpath_buf: *mut *mut libc::c_char,
) -> u8 {
	if binprefix.is_null() {
		return 1 as libc::c_int as u8;
	}
	while !paths.is_null() && !(*paths).is_null() {
		*binpath_buf = ft_strjoin(*paths, binprefix);
		if !binpath_buf.is_null() && (*binpath_buf).is_null() {
			return 1 as libc::c_int as u8;
		}
		if access(*binpath_buf, 0 as libc::c_int) == 0 as libc::c_int {
			return 0 as libc::c_int as u8;
		}
		free_null(binpath_buf as *mut libc::c_void);
		paths = paths.offset(1);
	}
	127 as libc::c_int as u8
}
unsafe extern "C" fn get_bin(
	mut paths: *mut *const libc::c_char,
	mut bin: *const libc::c_char,
	mut binpath_buf: *mut *mut libc::c_char,
) -> u8 {
	let mut bin_prefix: *const libc::c_char =
		ft_strjoin(b"/\0" as *const u8 as *const libc::c_char, bin);
	if bin_prefix.is_null() {
		arr_free(paths as *mut *mut libc::c_char);
		return 1 as libc::c_int as u8;
	}
	if *bin == 0 {
		free(bin_prefix as *mut libc::c_char as *mut libc::c_void);
		*binpath_buf = ft_strdup(b"\0" as *const u8 as *const libc::c_char);
		arr_free(paths as *mut *mut libc::c_char);
		return 127 as libc::c_int as u8;
	}
	let mut status: u8 = find_bin(paths, bin_prefix, binpath_buf);
	arr_free(paths as *mut *mut libc::c_char);
	free(bin_prefix as *mut libc::c_char as *mut libc::c_void);
	if status as libc::c_int == 0 as libc::c_int
		&& access(*binpath_buf, 1 as libc::c_int) == -(1 as libc::c_int)
	{
		return 126 as libc::c_int as u8;
	}
	status
}
#[no_mangle]
pub unsafe extern "C" fn set_single(
	mut env: *const *mut libc::c_char,
	mut bin: libc::c_char,
	mut binpath_buf: *mut *mut libc::c_char,
) -> u8 {
	if bin as libc::c_int == '/' as i32 {
		eprint(b"/: Is a directory\0" as *const u8 as *const libc::c_char);
		return 126 as libc::c_int as u8;
	}
	if bin as libc::c_int == '.' as i32 {
		eprint(
			b".: filename argument required\0" as *const u8 as *const libc::c_char,
			*binpath_buf,
		);
		return 2 as libc::c_int as u8;
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
	126 as libc::c_int as u8
}
#[no_mangle]
pub unsafe extern "C" fn set_binpath(
	mut env: *const *mut libc::c_char,
	mut bin: *const libc::c_char,
	mut binpath_buf: *mut *mut libc::c_char,
) -> u8 {
	if bin.is_null() {
		return 1 as libc::c_int as u8;
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
			return 127 as libc::c_int as u8;
		}
		if (*binpath_buf).is_null() {
			return 1 as libc::c_int as u8;
		}
		if access(*binpath_buf, 0 as libc::c_int) == -(1 as libc::c_int) {
			return 127 as libc::c_int as u8;
		}
		if access(*binpath_buf, 1 as libc::c_int) == -(1 as libc::c_int) {
			return 126 as libc::c_int as u8;
		}
		return 0 as libc::c_int as u8;
	}
	let mut path: *mut libc::c_char = get_env(env, b"PATH\0" as *const u8 as *const libc::c_char);
	let mut paths: *mut *const libc::c_char =
		ft_split(path, ':' as i32 as libc::c_char) as *mut *const libc::c_char;
	free(path as *mut libc::c_void);
	get_bin(paths, bin, binpath_buf)
}
