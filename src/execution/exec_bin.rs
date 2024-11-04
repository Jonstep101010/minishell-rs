use ::libc;
use libc::{exit, free, strerror};
use libft_rs::ft_strchr::ft_strchr;
use libutils_rs::src::array::arr_free::arr_free;
pub type size_t = libc::c_ulong;
use crate::{
	__errno_location, t_shell,
	tokenizer::{build_command::get_cmd_arr_token, destroy_tokens::destroy_all_tokens},
	utils::error::eprint,
};

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
use crate::utils::exit_free::exit_free;

use super::bin_path::set_binpath;
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
unsafe extern "C" fn execve_fail(mut shell: *mut t_shell, mut cmd: *mut libc::c_char) {
	eprint(
		b"%s: %s\0" as *const u8 as *const libc::c_char,
		cmd,
		strerror(*__errno_location()),
	);
	if !((*shell).env).is_null() {
		arr_free((*shell).env);
	}
	destroy_all_tokens(shell);
	free(shell as *mut libc::c_void);
	exit(*__errno_location());
}
#[no_mangle]
pub unsafe extern "C" fn exec_bin(mut shell: *mut t_shell, mut token: *mut t_token) -> libc::c_int {
	let mut access_status: libc::c_int = 0;
	let mut command: *mut *const libc::c_char =
		get_cmd_arr_token(token) as *mut *const libc::c_char;
	if command.is_null() {
		exit_free(shell, 0 as libc::c_int);
	}
	access_status = set_binpath((*shell).env, *command, &mut (*token).bin) as libc::c_int;
	if access_status == 1 as libc::c_int
		|| access_status == 2 as libc::c_int
		|| access_status == 126 as libc::c_int
		|| access_status == 127 as libc::c_int
	{
		if access_status == 126 as libc::c_int
			&& (ft_strchr(
				b"~/\0" as *const u8 as *const libc::c_char,
				**command as libc::c_int,
			))
			.is_null()
		{
			eprint(
				b"%s: %s\0" as *const u8 as *const libc::c_char,
				*command,
				strerror(*__errno_location()),
			);
		}
		if access_status == 127 as libc::c_int {
			eprint(
				b"%s: command not found\0" as *const u8 as *const libc::c_char,
				*command,
			);
		}
		arr_free(command as *mut *mut libc::c_char);
		exit_free(shell, access_status);
	}
	if libc::execve(
		(*token).bin,
		command as *mut *mut libc::c_char as *const *const libc::c_char,
		(*shell).env as *const *const libc::c_char,
	) == -(1 as libc::c_int)
	{
		arr_free(command as *mut *mut libc::c_char);
		execve_fail(shell, (*token).bin);
	}
	arr_free(command as *mut *mut libc::c_char);
	exit_free(shell, 0 as libc::c_int);
	0 as libc::c_int
}
