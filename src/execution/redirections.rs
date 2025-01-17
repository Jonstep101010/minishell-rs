use crate::prelude::*;
use crate::t_arg;
use ::libc;
use libc::{access, close, dup2, open};

unsafe fn open_redir(mut file: *const libc::c_char, mut redir: e_redir, mut fd: *mut i32) -> i32 {
	if redir as libc::c_uint == e_redir::INPUT_REDIR as i32 as libc::c_uint {
		let mut perm = access(file, 0_i32);
		if perm != 0_i32 {
			return 127_i32;
		}
		perm = access(file, 4_i32);
		if perm != 0_i32 {
			return 126_i32;
		}
		*fd = open(file, 0_i32);
	} else if redir as libc::c_uint == e_redir::OUTPUT_REDIR as i32 as libc::c_uint {
		*fd = open(file, 0o1_i32 | 0o100_i32 | 0o1000_i32, 0o644_i32);
	} else if redir as libc::c_uint == e_redir::APPEND as i32 as libc::c_uint {
		let mut perm = access(file, 2_i32);
		if perm != 0_i32 {
			todo!("handle missing perms in open_redir");
		}
		*fd = open(file, 0o1_i32 | 0o100_i32 | 0o2000_i32, 0o644_i32);
	}
	if *fd == -1_i32 {
		todo!("handle open failure");
	}
	0_i32
}
#[unsafe(no_mangle)]
pub unsafe fn do_redirections(
	mut cmd_args: *mut t_arg,
	mut error_elem: *mut *mut libc::c_char,
) -> i32 {
	let mut i: i32 = 0;
	while !((*cmd_args.offset(i as isize)).elem).is_null() {
		let mut fd = 0_i32;
		if (*cmd_args.offset(i as isize)).type_0 as libc::c_uint
			== e_arg::REDIR as i32 as libc::c_uint
			&& (*cmd_args.offset(i as isize)).redir as libc::c_uint
				!= e_redir::HEREDOC as i32 as libc::c_uint
		{
			if open_redir(
				(*cmd_args.offset(i as isize)).elem,
				(*cmd_args.offset(i as isize)).redir,
				&mut fd,
			) != 0_i32
			{
				*error_elem = (*cmd_args.offset(i as isize)).elem;
				todo!("handle open_redir failure");
			}
			if (*cmd_args.offset(i as isize)).redir as libc::c_uint
				!= e_redir::INPUT_REDIR as i32 as libc::c_uint
			{
				dup2(fd, 1_i32);
			} else {
				dup2(fd, 0_i32);
			}
			close(fd);
		}
		i += 1;
	}
	0_i32
}
