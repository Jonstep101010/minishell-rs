use ::libc;
use libc::{exit, free, strerror};

use crate::{
	__errno_location, libutils_rs::src::array::arr_free::arr_free, t_shell,
	tokenizer::destroy_tokens::destroy_all_tokens, utils::error::eprint,
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
#[no_mangle]
pub unsafe extern "C" fn exit_free(mut shell: *mut t_shell, mut exit_code: libc::c_int) {
	if !((*shell).env).is_null() {
		arr_free((*shell).env);
	}
	destroy_all_tokens(shell);
	free(shell as *mut libc::c_void);
	exit(exit_code);
}
#[no_mangle]
pub unsafe extern "C" fn exit_error(mut shell: *mut t_shell, mut error_elem: *mut libc::c_char) {
	let mut error: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
	error = strerror(*__errno_location());
	if !error_elem.is_null() {
		eprint(
			b"%s: %s\0" as *const u8 as *const libc::c_char,
			error_elem,
			error,
		);
	}
	if !((*shell).env).is_null() {
		arr_free((*shell).env);
	}
	destroy_all_tokens(shell);
	free(shell as *mut libc::c_void);
	exit(*__errno_location());
}
