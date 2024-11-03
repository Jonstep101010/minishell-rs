use ::libc;
extern "C" {
	fn init_token(size: size_t) -> *mut t_token;
	fn arr_len(arr: *const *mut libc::c_char) -> size_t;
	fn arr_free(arr: *mut *mut libc::c_char);
	fn append_str_arr_free(
		arr: *mut *mut libc::c_char,
		s: *mut libc::c_char,
	) -> *mut *mut libc::c_char;
	fn eprint(fmt: *const libc::c_char, _: ...);
	fn free(_: *mut libc::c_void);
	fn ft_strdup(s: *const libc::c_char) -> *mut libc::c_char;
	fn split_outside_quotes(
		to_split: *const libc::c_char,
		set: *const libc::c_char,
	) -> *mut *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
use crate::t_token;

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
pub unsafe extern "C" fn get_cmd_arr_token(mut token: *mut t_token) -> *mut *mut libc::c_char {
	let mut i: libc::c_int = 0;
	let mut cmd_arr: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
	i = 0 as libc::c_int;
	cmd_arr = 0 as *mut *mut libc::c_char;
	if token.is_null() || ((*token).cmd_args).is_null() {
		return 0 as *mut *mut libc::c_char;
	}
	if !((*((*token).cmd_args).offset(0 as libc::c_int as isize)).elem).is_null() {
		while !((*((*token).cmd_args).offset(i as isize)).elem).is_null() {
			if (*((*token).cmd_args).offset(i as isize)).type_0 as libc::c_uint
				!= REDIR as libc::c_int as libc::c_uint
			{
				cmd_arr = append_str_arr_free(
					cmd_arr,
					ft_strdup((*((*token).cmd_args).offset(i as isize)).elem),
				);
				if cmd_arr.is_null() {
					return 0 as *mut *mut libc::c_char;
				}
			}
			i += 1;
		}
	}
	return cmd_arr;
}
#[no_mangle]
pub unsafe extern "C" fn get_tokens(mut trimmed_line: *const libc::c_char) -> *mut t_token {
	let mut i: libc::c_int = 0;
	let mut split_pipes: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
	let mut token: *mut t_token = 0 as *mut t_token;
	split_pipes = split_outside_quotes(trimmed_line, b"|\0" as *const u8 as *const libc::c_char);
	if split_pipes.is_null() {
		eprint(b"alloc fail!\0" as *const u8 as *const libc::c_char);
		return 0 as *mut libc::c_void as *mut t_token;
	}
	if (*split_pipes).is_null() {
		arr_free(split_pipes);
		return 0 as *mut libc::c_void as *mut t_token;
	}
	token = init_token(arr_len(split_pipes));
	if token.is_null() {
		eprint(b"alloc fail token\0" as *const u8 as *const libc::c_char);
	}
	i = 0 as libc::c_int;
	while !token.is_null() && !(*split_pipes.offset(i as isize)).is_null() {
		let ref mut fresh0 = (*token.offset(i as isize)).split_pipes;
		*fresh0 = *split_pipes.offset(i as isize);
		i += 1;
	}
	free(split_pipes as *mut libc::c_void);
	return token;
}
