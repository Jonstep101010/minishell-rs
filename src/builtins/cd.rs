use ::libc;

use crate::{
	__errno_location,
	environment::{export_env::export_env, get_env::get_env},
	t_shell, t_token,
	tokenizer::build_command::get_cmd_arr_token,
	utils::error::eprint,
};
use libutils_rs::src::{
	array::arr_free::arr_free,
	string::join_strings::{free_first_join, free_second_join},
	utils::free_mem::free_null,
};

use libc::{chdir, free, getcwd, strerror};

unsafe extern "C" fn changedir(
	mut path: *const libc::c_char,
	mut shell: *mut t_shell,
) -> libc::c_int {
	let mut oldpwd: *mut libc::c_char = getcwd(
		std::ptr::null_mut::<libc::c_char>(),
		0 as libc::c_int as usize,
	);
	if chdir(path) == -(1 as libc::c_int) {
		eprint(
			b"cd: %s: %s\0" as *const u8 as *const libc::c_char,
			path,
			strerror(*__errno_location()),
		);
		free(oldpwd as *mut libc::c_void);
		return -(1 as libc::c_int);
	}
	let mut pwd: *mut libc::c_char = getcwd(
		std::ptr::null_mut::<libc::c_char>(),
		0 as libc::c_int as usize,
	);
	if pwd.is_null() {
		eprint(
			b"cd: %s: %s\0" as *const u8 as *const libc::c_char,
			path,
			strerror(*__errno_location()),
		);
		free(oldpwd as *mut libc::c_void);
		return -(1 as libc::c_int);
	}
	export_env(
		shell,
		free_second_join(b"PWD=\0" as *const u8 as *const libc::c_char, pwd),
	);
	export_env(
		shell,
		free_second_join(b"OLDPWD=\0" as *const u8 as *const libc::c_char, oldpwd),
	);
	0 as libc::c_int
}
unsafe extern "C" fn cd_internal(
	mut cmd_args: *mut *const libc::c_char,
	mut shell: *mut t_shell,
) -> libc::c_int {
	let mut path: *mut libc::c_char =
		get_env((*shell).env, b"HOME\0" as *const u8 as *const libc::c_char);
	let mut oldpwd: *mut libc::c_char = get_env(
		(*shell).env,
		b"OLDPWD\0" as *const u8 as *const libc::c_char,
	);
	if (*cmd_args.offset(1 as libc::c_int as isize)).is_null() && path.is_null() {
		free_null(&mut oldpwd as *mut *mut libc::c_char as *mut libc::c_void);
		eprint(b"cd: HOME not set\0" as *const u8 as *const libc::c_char);
		return 1 as libc::c_int;
	}
	if (*cmd_args.offset(1 as libc::c_int as isize)).is_null() && !path.is_null() {
		changedir(path, shell);
	} else if **cmd_args.offset(1 as libc::c_int as isize) as libc::c_int == '~' as i32
		&& !path.is_null()
	{
		path = free_first_join(
			path,
			(*cmd_args.offset(1 as libc::c_int as isize)).offset(1 as libc::c_int as isize),
		);
		if changedir(path, shell) == -(1 as libc::c_int) {
			free(path as *mut libc::c_void);
			free(oldpwd as *mut libc::c_void);
			return 1 as libc::c_int;
		}
	} else if **cmd_args.offset(1 as libc::c_int as isize) as libc::c_int == '-' as i32
		&& !oldpwd.is_null()
	{
		oldpwd = free_first_join(
			oldpwd,
			(*cmd_args.offset(1 as libc::c_int as isize)).offset(1 as libc::c_int as isize),
		);
		if changedir(oldpwd, shell) == -(1 as libc::c_int) {
			free(path as *mut libc::c_void);
			free(oldpwd as *mut libc::c_void);
			return 1 as libc::c_int;
		}
	} else if changedir(*cmd_args.offset(1 as libc::c_int as isize), shell) == -(1 as libc::c_int) {
		free(path as *mut libc::c_void);
		free(oldpwd as *mut libc::c_void);
		return 1 as libc::c_int;
	}
	free(path as *mut libc::c_void);
	free(oldpwd as *mut libc::c_void);
	0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn builtin_cd(
	mut shell: *mut t_shell,
	mut token: *mut t_token,
) -> libc::c_int {
	let mut command: *mut *const libc::c_char =
		get_cmd_arr_token(token) as *mut *const libc::c_char;
	let status = cd_internal(command, shell);
	arr_free(command as *mut *mut libc::c_char);
	status
}
