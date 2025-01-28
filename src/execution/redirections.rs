use crate::prelude::*;
use ::libc;
use libc::{access, close, dup2, open};

pub fn do_redirections(cmd_args: &mut [t_arg]) -> Result<(), i32> {
	let mut i = 0;
	while i < cmd_args.len() {
		let mut fd = 0;
		if (cmd_args[i]).type_0 == REDIR && (cmd_args[i]).redir != Some(HEREDOC) {
			let file: *const libc::c_char = (cmd_args[i]).elem;
			match (cmd_args[i]).redir.unwrap() {
				INPUT_REDIR => {
					let mut perm = unsafe { access(file, 0) };
					if perm != 0 {
						return Err(127);
					}
					perm = unsafe { access(file, 4) };
					if perm != 0 {
						return Err(126);
					}
					fd = unsafe { open(file, 0) };
				}
				OUTPUT_REDIR => {
					fd = unsafe { open(file, 0o1 | 0o100 | 0o1000, 0o644) };
				}
				APPEND => {
					let perm = unsafe { access(file, 2) };
					if perm != 0 {
						todo!("handle missing perms in open_redir");
					}
					fd = unsafe { open(file, 0o1 | 0o100 | 0o2000, 0o644) };
				}
				_ => {}
			}
			if fd == -1 {
				// todo!("handle open failure");
				let tmp = unsafe { std::ffi::CStr::from_ptr((cmd_args[i]).elem) };
				eprint_msh!("failed to execute: {}", tmp.to_str().unwrap());
				return Err(-1);
			}
			if (cmd_args[i]).redir != Some(INPUT_REDIR) {
				unsafe { dup2(fd, 1) };
			} else {
				unsafe { dup2(fd, 0) };
			}
			unsafe { close(fd) };
		}
		i += 1;
	}
	Ok(())
}
