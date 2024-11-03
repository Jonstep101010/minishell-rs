use ::libc;
extern "C" {
	fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
	fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
	fn close(__fd: libc::c_int) -> libc::c_int;
	fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
	fn __errno_location() -> *mut libc::c_int;
}
use crate::t_arg;
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
unsafe extern "C" fn open_redir(
	mut file: *const libc::c_char,
	mut redir: e_redir,
	mut fd: *mut libc::c_int,
) -> libc::c_int {
	let mut perm: libc::c_int = 0;
	if redir as libc::c_uint == INPUT_REDIR as libc::c_int as libc::c_uint {
		perm = access(file, 0 as libc::c_int);
		if perm != 0 as libc::c_int {
			return 127 as libc::c_int;
		}
		perm = access(file, 4 as libc::c_int);
		if perm != 0 as libc::c_int {
			return 126 as libc::c_int;
		}
		*fd = open(file, 0 as libc::c_int);
	} else if redir as libc::c_uint == OUTPUT_REDIR as libc::c_int as libc::c_uint {
		*fd = open(
			file,
			0o1 as libc::c_int | 0o100 as libc::c_int | 0o1000 as libc::c_int,
			0o644 as libc::c_int,
		);
	} else if redir as libc::c_uint == APPEND as libc::c_int as libc::c_uint {
		perm = access(file, 2 as libc::c_int);
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
	return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn do_redirections(
	mut cmd_args: *mut t_arg,
	mut error_elem: *mut *mut libc::c_char,
) -> libc::c_int {
	let mut i: libc::c_int = 0;
	let mut fd: libc::c_int = 0;
	i = 0 as libc::c_int;
	while !((*cmd_args.offset(i as isize)).elem).is_null() {
		fd = 0 as libc::c_int;
		if (*cmd_args.offset(i as isize)).type_0 as libc::c_uint
			== REDIR as libc::c_int as libc::c_uint
			&& (*cmd_args.offset(i as isize)).redir as libc::c_uint
				!= HEREDOC as libc::c_int as libc::c_uint
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
				!= INPUT_REDIR as libc::c_int as libc::c_uint
			{
				dup2(fd, 1 as libc::c_int);
			} else {
				dup2(fd, 0 as libc::c_int);
			}
			close(fd);
		}
		i += 1;
	}
	return 0 as libc::c_int;
}
