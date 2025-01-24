use crate::prelude::*;
use crate::t_arg;
use ::libc;
use libc::{access, close, dup2, open};

unsafe fn open_redir(mut file: *const libc::c_char, mut redir: e_redir, mut fd: *mut i32) -> i32 {
	if redir as libc::c_uint == e_redir::INPUT_REDIR as i32 as libc::c_uint {
		let mut perm = access(file, 0);
		if perm != 0 {
			return 127;
		}
		perm = access(file, 4);
		if perm != 0 {
			return 126;
		}
		*fd = open(file, 0);
	} else if redir as libc::c_uint == e_redir::OUTPUT_REDIR as i32 as libc::c_uint {
		*fd = open(file, 0o1 | 0o100 | 0o1000, 0o644);
	} else if redir as libc::c_uint == e_redir::APPEND as i32 as libc::c_uint {
		let mut perm = access(file, 2);
		if perm != 0 {
			todo!("handle missing perms in open_redir");
		}
		*fd = open(file, 0o1 | 0o100 | 0o2000, 0o644);
	}
	if *fd == -1 {
		todo!("handle open failure");
	}
	0
}
#[unsafe(no_mangle)]
pub unsafe fn do_redirections(mut cmd_args: *mut t_arg) -> i32 {
	let mut i = 0;
	while !((*cmd_args.add(i)).elem).is_null() {
		let mut fd = 0;
		if (*cmd_args.add(i)).type_0 == e_arg::REDIR
			&& (*cmd_args.add(i)).redir != Some(e_redir::HEREDOC)
		{
			if open_redir(
				(*cmd_args.add(i)).elem,
				(*cmd_args.add(i)).redir.unwrap(),
				&mut fd,
			) != 0
			{
				let tmp = std::ffi::CStr::from_ptr((*cmd_args.add(i)).elem);
				eprint_msh!("failed to execute: {}", tmp.to_str().unwrap());
				return 1;
			}
			if (*cmd_args.add(i)).redir != Some(e_redir::INPUT_REDIR) {
				dup2(fd, 1);
			} else {
				dup2(fd, 0);
			}
			close(fd);
		}
		i += 1;
	}
	0
}
