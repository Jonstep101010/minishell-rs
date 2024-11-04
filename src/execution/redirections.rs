use crate::prelude::*;
use crate::{__errno_location, t_arg};
use ::libc;
use libc::{access, close, dup2, open};

unsafe extern "C" fn open_redir(
	mut file: *const libc::c_char,
	mut redir: e_redir,
	mut fd: *mut libc::c_int,
) -> libc::c_int {
	if redir as libc::c_uint == e_redir::INPUT_REDIR as libc::c_int as libc::c_uint {
		let mut perm = access(file, 0 as libc::c_int);
		if perm != 0 as libc::c_int {
			return 127 as libc::c_int;
		}
		perm = access(file, 4 as libc::c_int);
		if perm != 0 as libc::c_int {
			return 126 as libc::c_int;
		}
		*fd = open(file, 0 as libc::c_int);
	} else if redir as libc::c_uint == e_redir::OUTPUT_REDIR as libc::c_int as libc::c_uint {
		*fd = open(
			file,
			0o1 as libc::c_int | 0o100 as libc::c_int | 0o1000 as libc::c_int,
			0o644 as libc::c_int,
		);
	} else if redir as libc::c_uint == e_redir::APPEND as libc::c_int as libc::c_uint {
		let mut perm = access(file, 2 as libc::c_int);
		if perm != 0 as libc::c_int {
			return *__errno_location();
		}
		*fd = open(
			file,
			0o1 as libc::c_int | 0o100 as libc::c_int | 0o2000 as libc::c_int,
			0o644 as libc::c_int,
		);
	}
	if *fd == -(1 as libc::c_int) {
		return *__errno_location();
	}
	0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn do_redirections(
	mut cmd_args: *mut t_arg,
	mut error_elem: *mut *mut libc::c_char,
) -> libc::c_int {
	let mut i: libc::c_int = 0;
	while !((*cmd_args.offset(i as isize)).elem).is_null() {
		let mut fd = 0 as libc::c_int;
		if (*cmd_args.offset(i as isize)).type_0 as libc::c_uint
			== e_arg::REDIR as libc::c_int as libc::c_uint
			&& (*cmd_args.offset(i as isize)).redir as libc::c_uint
				!= e_redir::HEREDOC as libc::c_int as libc::c_uint
		{
			if open_redir(
				(*cmd_args.offset(i as isize)).elem,
				(*cmd_args.offset(i as isize)).redir,
				&mut fd,
			) != 0 as libc::c_int
			{
				*error_elem = (*cmd_args.offset(i as isize)).elem;
				return *__errno_location();
			}
			if (*cmd_args.offset(i as isize)).redir as libc::c_uint
				!= e_redir::INPUT_REDIR as libc::c_int as libc::c_uint
			{
				dup2(fd, 1 as libc::c_int);
			} else {
				dup2(fd, 0 as libc::c_int);
			}
			close(fd);
		}
		i += 1;
	}
	0 as libc::c_int
}
