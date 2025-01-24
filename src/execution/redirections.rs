use crate::e_arg::*;
use crate::e_redir::*;
use crate::prelude::*;
use crate::t_arg;
use ::libc;
use libc::{access, close, dup2, open};

pub unsafe fn do_redirections(mut cmd_args: *mut t_arg) -> Result<(), i32> {
	let mut i = 0;
	while !((*cmd_args.add(i)).elem).is_null() {
		let mut fd = 0;
		if (*cmd_args.add(i)).type_0 == REDIR && (*cmd_args.add(i)).redir != Some(HEREDOC) {
			let mut file: *const libc::c_char = (*cmd_args.add(i)).elem;
			match (*cmd_args.add(i)).redir.unwrap() {
				INPUT_REDIR => {
					let mut perm = access(file, 0);
					if perm != 0 {
						return Err(127);
					}
					perm = access(file, 4);
					if perm != 0 {
						return Err(126);
					}
					fd = open(file, 0);
				}
				OUTPUT_REDIR => {
					fd = open(file, 0o1 | 0o100 | 0o1000, 0o644);
				}
				APPEND => {
					let mut perm = access(file, 2);
					if perm != 0 {
						todo!("handle missing perms in open_redir");
					}
					fd = open(file, 0o1 | 0o100 | 0o2000, 0o644);
				}
				_ => {}
			}
			if fd == -1 {
				// todo!("handle open failure");
				let tmp = std::ffi::CStr::from_ptr((*cmd_args.add(i)).elem);
				eprint_msh!("failed to execute: {}", tmp.to_str().unwrap());
				return Err(-1);
			}
			if (*cmd_args.add(i)).redir != Some(INPUT_REDIR) {
				dup2(fd, 1);
			} else {
				dup2(fd, 0);
			}
			close(fd);
		}
		i += 1;
	}
	Ok(())
}
