use ::libc;

use crate::t_shell;
extern "C" {
	fn free_null(p: *mut libc::c_void);
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
use crate::t_arg;
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
#[no_mangle]
pub unsafe extern "C" fn destroy_all_tokens(mut shell: *mut t_shell) {
	let mut token: *mut t_token = 0 as *mut t_token;
	let mut i: size_t = 0;
	let mut ii: size_t = 0;
	token = (*shell).token;
	i = 0 as libc::c_int as size_t;
	while !token.is_null() && i < (*shell).token_len {
		if !((*token.offset(i as isize)).cmd_args).is_null() {
			ii = 0 as libc::c_int as size_t;
			while !((*((*token.offset(i as isize)).cmd_args).offset(ii as isize)).elem).is_null() {
				free_null(
					&mut (*((*token.offset(i as isize)).cmd_args).offset(ii as isize)).elem
						as *mut *mut libc::c_char as *mut libc::c_void,
				);
				ii = ii.wrapping_add(1);
				ii;
			}
			free_null(
				&mut (*token.offset(i as isize)).cmd_args as *mut *mut t_arg as *mut libc::c_void,
			);
		}
		if !((*token.offset(i as isize)).bin).is_null() {
			free_null(
				&mut (*token.offset(i as isize)).bin as *mut *mut libc::c_char as *mut libc::c_void,
			);
		}
		i = i.wrapping_add(1);
		i;
	}
	free_null(&mut (*shell).token as *mut *mut t_token as *mut libc::c_void);
	(*shell).token_len = 0 as libc::c_int as size_t;
}
