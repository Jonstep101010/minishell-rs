use ::libc;
extern "C" {
	fn get_cmd_arr_token(token: *mut t_token) -> *mut *mut libc::c_char;
	fn check_valid_key(s: *const libc::c_char) -> bool;
	fn export_env(shell: *mut t_shell, key_val: *mut libc::c_char);
	fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
	fn str_cchr(s: *const libc::c_char, c: libc::c_char) -> libc::c_int;
	fn arr_free(arr: *mut *mut libc::c_char);
	fn eprint(fmt: *const libc::c_char, _: ...);
	fn ft_strdup(s: *const libc::c_char) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
use crate::{t_shell, t_token};

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
unsafe extern "C" fn declare_x_env_var(mut env: *const *mut libc::c_char) -> libc::c_int {
	while !(*env).is_null() {
		printf(
			b"declare -x %s\n\0" as *const u8 as *const libc::c_char,
			*env,
		);
		env = env.offset(1);
	}
	return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn builtin_export(
	mut shell: *mut t_shell,
	mut token: *mut t_token,
) -> libc::c_int {
	let mut command: *mut *const libc::c_char =
		get_cmd_arr_token(token) as *mut *const libc::c_char;
	let mut i: size_t = 0;
	i = 1 as libc::c_int as size_t;
	if command.is_null() || (*command.offset(i as isize)).is_null() {
		arr_free(command as *mut *mut libc::c_char);
		return declare_x_env_var((*shell).env);
	}
	while !(*command.offset(i as isize)).is_null() {
		if !check_valid_key(*command.offset(i as isize)) {
			eprint(
				b"export: `%s': not a valid identifier\0" as *const u8 as *const libc::c_char,
				*command.offset(i as isize),
			);
			arr_free(command as *mut *mut libc::c_char);
			return 1 as libc::c_int;
		}
		if str_cchr(*command.offset(i as isize), '=' as i32 as libc::c_char) >= 1 as libc::c_int {
			export_env(shell, ft_strdup(*command.offset(i as isize)));
		}
		i = i.wrapping_add(1);
	}
	arr_free(command as *mut *mut libc::c_char);
	if i > 1 as libc::c_int as libc::c_ulong {
		return 0 as libc::c_int;
	}
	return 1 as libc::c_int;
}
