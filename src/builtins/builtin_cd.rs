use ::libc;

use crate::t_shell;
extern "C" {
	fn arr_free(arr: *mut *mut libc::c_char);
	fn free_first_join(s1: *mut libc::c_char, s2: *const libc::c_char) -> *mut libc::c_char;
	fn free_second_join(s1: *const libc::c_char, s2: *mut libc::c_char) -> *mut libc::c_char;
	fn free_null(p: *mut libc::c_void);
	fn get_cmd_arr_token(token: *mut t_token) -> *mut *mut libc::c_char;
	fn eprint(fmt: *const libc::c_char, _: ...);
	fn get_env(env: *const *mut libc::c_char, key: *const libc::c_char) -> *mut libc::c_char;
	fn export_env(shell: *mut t_shell, key_val: *mut libc::c_char);
	fn __errno_location() -> *mut libc::c_int;
	fn free(_: *mut libc::c_void);
	fn strerror(_: libc::c_int) -> *mut libc::c_char;
	fn chdir(__path: *const libc::c_char) -> libc::c_int;
	fn getcwd(__buf: *mut libc::c_char, __size: size_t) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct termios {
	pub c_iflag: tcflag_t,
	pub c_oflag: tcflag_t,
	pub c_cflag: tcflag_t,
	pub c_lflag: tcflag_t,
	pub c_line: cc_t,
	pub c_cc: [cc_t; 32],
	pub c_ispeed: speed_t,
	pub c_ospeed: speed_t,
}
pub type speed_t = libc::c_uint;
pub type cc_t = libc::c_uchar;
pub type tcflag_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type __uint8_t = libc::c_uchar;
use crate::t_token;
pub type e_redir = libc::c_uint;
pub const HEREDOC: e_redir = 4;
pub const APPEND: e_redir = 3;
pub const OUTPUT_REDIR: e_redir = 2;
pub const INPUT_REDIR: e_redir = 1;
pub const NO_REDIR: e_redir = 0;
pub type e_arg = libc::c_uint;
pub const REDIR_REMOVED: e_arg = 2;
pub const REDIR: e_arg = 1;
pub const STRING: e_arg = 0;
unsafe extern "C" fn changedir(
	mut path: *const libc::c_char,
	mut shell: *mut t_shell,
) -> libc::c_int {
	let mut pwd: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
	let mut oldpwd: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
	oldpwd = getcwd(
		std::ptr::null_mut::<libc::c_char>(),
		0 as libc::c_int as size_t,
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
	pwd = getcwd(
		std::ptr::null_mut::<libc::c_char>(),
		0 as libc::c_int as size_t,
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
	let mut path: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
	let mut oldpwd: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
	path = get_env((*shell).env, b"HOME\0" as *const u8 as *const libc::c_char);
	oldpwd = get_env(
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
	let mut status: libc::c_int = 0;
	let mut command: *mut *const libc::c_char =
		get_cmd_arr_token(token) as *mut *const libc::c_char;
	status = cd_internal(command, shell);
	arr_free(command as *mut *mut libc::c_char);
	status
}
