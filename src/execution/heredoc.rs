use ::libc;
use libft_rs::{ft_putendl_fd::ft_putendl_fd, ft_strchr::ft_strchr};
use libutils_rs::src::string::str_equal::equal;
extern "C" {
	fn strerror(_: libc::c_int) -> *mut libc::c_char;
	fn readline(_: *const libc::c_char) -> *mut libc::c_char;
	fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
	fn close(__fd: libc::c_int) -> libc::c_int;
	fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
	fn unlink(__name: *const libc::c_char) -> libc::c_int;
	fn __errno_location() -> *mut libc::c_int;
	fn free(_: *mut libc::c_void);
	fn exit(_: libc::c_int) -> !;
	// fn ft_strchr(str: *const libc::c_char, c: libc::c_int) -> *mut libc::c_char;
	// fn ft_putendl_fd(s: *mut libc::c_char, fd: libc::c_int);
	// fn equal(expected: *const libc::c_char, actual: *const libc::c_char) -> *mut libc::c_char;
	fn eprint(fmt: *const libc::c_char, _: ...);
	// fn expander(
	// 	input_expander: *const libc::c_char,
	// 	env: *const *mut libc::c_char,
	// ) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;

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
pub static mut g_ctrl_c: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn heredoc_loop(
	mut delim: *mut libc::c_char,
	mut fd: libc::c_int,
	mut env: *mut *mut libc::c_char,
) {
	let mut expanded: *mut libc::c_char = 0 as *mut libc::c_char;
	let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
	line = 0 as *mut libc::c_char;
	g_ctrl_c = 0 as libc::c_int;
	while 1 as libc::c_int != 0 && g_ctrl_c == 0 {
		line = readline(b"> \0" as *const u8 as *const libc::c_char);
		if !(equal(delim, line)).is_null() || line.is_null() || g_ctrl_c != 0 {
			g_ctrl_c = 0 as libc::c_int;
			break;
		} else {
			if !(ft_strchr(line, '$' as i32)).is_null() {
				expanded = crate::environment::expander::expander(line, env);
				if !expanded.is_null() && (equal(expanded, line)).is_null() {
					ft_putendl_fd(expanded, fd);
				}
				free(expanded as *mut libc::c_void);
			} else {
				ft_putendl_fd(line, fd);
			}
			free(line as *mut libc::c_void);
		}
	}
	free(line as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn do_heredocs(
	mut token: *mut t_token,
	mut target: *mut libc::c_int,
	mut env: *mut *mut libc::c_char,
) {
	let mut fd: libc::c_int = 0;
	let mut i: libc::c_int = 0;
	i = -(1 as libc::c_int);
	loop {
		i += 1;
		if ((*((*token).cmd_args).offset(i as isize)).elem).is_null() {
			break;
		}
		if (*((*token).cmd_args).offset(i as isize)).redir as libc::c_uint
			== HEREDOC as libc::c_int as libc::c_uint
		{
			fd = open(
				b".heredoc.txt\0" as *const u8 as *const libc::c_char,
				0o2 as libc::c_int | 0o100 as libc::c_int | 0o1000 as libc::c_int,
				0o644 as libc::c_int,
			);
			if fd == -(1 as libc::c_int) {
				eprint(
					b"%s\0" as *const u8 as *const libc::c_char,
					strerror(*__errno_location()),
				);
				return exit(1 as libc::c_int);
			}
			heredoc_loop((*((*token).cmd_args).offset(i as isize)).elem, fd, env);
			close(fd);
			fd = open(
				b".heredoc.txt\0" as *const u8 as *const libc::c_char,
				0 as libc::c_int,
			);
			dup2(fd, *target);
			close(fd);
			unlink(b".heredoc.txt\0" as *const u8 as *const libc::c_char);
		}
	}
}
