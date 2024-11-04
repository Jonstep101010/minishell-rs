use ::libc;

use libft_rs::ft_strchr::ft_strchr;
use libutils_rs::src::array::{arr_free::arr_free, remove_str::rm_str_arr};

use crate::{
	environment::{check_key::check_valid_key, get_index::get_index_env},
	t_shell, t_token,
	tokenizer::build_command::get_cmd_arr_token,
	utils::error::eprint,
};

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
unsafe extern "C" fn check_illegal_char(mut str: *const libc::c_char) -> bool {
	while *str != 0 {
		if !(ft_strchr(
			b"?=;$.:><[]()/+-{}*#@!^\0" as *const u8 as *const libc::c_char,
			*str as libc::c_int,
		))
		.is_null()
		{
			return 1 as libc::c_int != 0;
		}
		str = str.offset(1);
	}
	0 as libc::c_int != 0
}
unsafe extern "C" fn unset_internal(
	mut args: *const *const libc::c_char,
	mut env: *mut *mut libc::c_char,
) -> libc::c_int {
	let mut index: libc::c_int = 0;
	while !(*args).is_null() {
		if !check_valid_key(*args) || check_illegal_char(*args) as libc::c_int != 0 {
			eprint(
				b"unset: `%s': not a valid identifier\0" as *const u8 as *const libc::c_char,
				*args,
			);
			return 1 as libc::c_int;
		}
		index = get_index_env(env, *args);
		if index >= 0 as libc::c_int && !(*env.offset(index as isize)).is_null() {
			rm_str_arr(env, *env.offset(index as isize));
		}
		args = args.offset(1);
	}
	0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn builtin_unset(
	mut shell: *mut t_shell,
	mut token: *mut t_token,
) -> libc::c_int {
	let mut status: libc::c_int = 0;
	let mut args: *mut *const libc::c_char = get_cmd_arr_token(token) as *mut *const libc::c_char;
	if ((*shell).env).is_null() || args.is_null() || (*(*shell).env).is_null() {
		return 0 as libc::c_int;
	}
	if (*args).is_null()
		|| (*args.offset(1 as libc::c_int as isize)).is_null()
		|| **args.offset(1 as libc::c_int as isize) == 0
	{
		arr_free(args as *mut *mut libc::c_char);
		return 0 as libc::c_int;
	}
	status = unset_internal(args.offset(1 as libc::c_int as isize), (*shell).env);
	arr_free(args as *mut *mut libc::c_char);
	status
}
