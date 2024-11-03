use ::libc;

use crate::builtins::builtin_exit::builtin_exit;
use crate::t_shell;
extern "C" {
	fn free(_: *mut libc::c_void);
	fn ft_itoa(n: libc::c_int) -> *mut libc::c_char;
	fn get_index_env(env: *const *mut libc::c_char, substr: *const libc::c_char) -> libc::c_int;
	fn append_str_arr_free(
		arr: *mut *mut libc::c_char,
		s: *mut libc::c_char,
	) -> *mut *mut libc::c_char;
	fn eprint(fmt: *const libc::c_char, _: ...);
	// fn builtin_exit(shell: *mut t_shell, nullable: *mut t_token) -> libc::c_int;
	fn free_second_join(s1: *const libc::c_char, s2: *mut libc::c_char) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct s_token {
// 	pub cmd_args: *mut t_arg,
// 	pub has_redir: bool,
// 	pub split_pipes: *mut libc::c_char,
// 	pub tmp_arr: *mut *mut libc::c_char,
// 	pub bin: *mut libc::c_char,
// 	pub cmd_func: Option<unsafe extern "C" fn(*mut t_shell, *mut t_token) -> libc::c_int>,
// }
// pub type t_token = s_token;
// pub type t_shell = s_shell;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct s_shell {
// 	pub exit_status: uint8_t,
// 	pub env: *mut *mut libc::c_char,
// 	pub token: *mut t_token,
// 	pub token_len: size_t,
// 	pub p_termios: termios,
// }
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
unsafe extern "C" fn update_var(mut env: *mut *mut libc::c_char, mut key_val: *mut libc::c_char) {
	if key_val.is_null() || env.is_null() || (*env).is_null() || *key_val == 0 {
		return;
	}
	free(*env as *mut libc::c_void);
	*env = key_val;
}
#[no_mangle]
pub unsafe extern "C" fn export_env(mut shell: *mut t_shell, mut key_val: *mut libc::c_char) {
	let mut index: libc::c_int = 0;
	if key_val.is_null() || *key_val == 0 {
		return eprint(
			b"export: malloc fail creating key_val\n\0" as *const u8 as *const libc::c_char,
		);
	}
	if shell.is_null()
		|| ((*shell).env).is_null()
		|| (*(*shell).env).is_null()
		|| **(*shell).env == 0
	{
		free(key_val as *mut libc::c_void);
		(*shell).exit_status = 1 as libc::c_int as uint8_t;
		eprint(b"fatal: invalid memory!\n\0" as *const u8 as *const libc::c_char);
		builtin_exit(shell, 0 as *mut t_token);
	}
	index = get_index_env((*shell).env, key_val);
	if index == -(1 as libc::c_int) {
		(*shell).env = append_str_arr_free((*shell).env, key_val);
	} else {
		update_var(&mut *((*shell).env).offset(index as isize), key_val);
	}
	if ((*shell).env).is_null() {
		(*shell).exit_status = 1 as libc::c_int as uint8_t;
		eprint(b"fatal: environment invalidated\n\0" as *const u8 as *const libc::c_char);
		builtin_exit(shell, 0 as *mut t_token);
	}
}
#[no_mangle]
pub unsafe extern "C" fn update_exit_status(mut shell: *mut t_shell, mut status: libc::c_int) {
	export_env(
		shell,
		free_second_join(b"?=\0" as *const u8 as *const libc::c_char, ft_itoa(status)),
	);
	(*shell).exit_status = status as uint8_t;
}
