use ::libc;
use gnu_readline_sys::readline;
#[allow(unused_imports)]
use libc::{close, dup2, free, open, strerror, unlink};

use libft_rs::{ft_putendl_fd::ft_putendl_fd, ft_strchr::ft_strchr};
use libutils_rs::src::string::str_equal::equal;

#[allow(unused_imports)]
use crate::{__errno_location, e_redir, prelude::*, t_token};

#[no_mangle]
pub static mut g_ctrl_c: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn heredoc_loop(
	mut delim: *mut libc::c_char,
	mut fd: libc::c_int,
	env: *const *const libc::c_char,
) {
	let mut line: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
	g_ctrl_c = 0 as libc::c_int;
	while 1 as libc::c_int != 0 && g_ctrl_c == 0 {
		line = readline(b"> \0" as *const u8 as *const libc::c_char);
		if !(equal(delim, line)).is_null() || line.is_null() || g_ctrl_c != 0 {
			g_ctrl_c = 0 as libc::c_int;
			break;
		} else {
			if !(ft_strchr(line, '$' as i32)).is_null() {
				let mut expanded: *mut libc::c_char =
					crate::environment::expander::expander(line, env);
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
	env: *const *const libc::c_char,
) {
	let mut i: libc::c_int = -1;
	loop {
		i += 1;
		if ((*((*token).cmd_args).offset(i as isize)).elem).is_null() {
			break;
		}
		if (*((*token).cmd_args).offset(i as isize)).redir as libc::c_uint
			== e_redir::HEREDOC as libc::c_int as libc::c_uint
		{
			let mut fd = open(
				b".heredoc.txt\0" as *const u8 as *const libc::c_char,
				0o2 as libc::c_int | 0o100 as libc::c_int | 0o1000 as libc::c_int,
				0o644 as libc::c_int,
			);
			if fd == -(1 as libc::c_int) {
				let err = stringify!(strerror(*__errno_location()));
				eprint_msh!("{}", err);
				panic!("i/o error");
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
