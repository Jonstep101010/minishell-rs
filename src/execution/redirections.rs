use crate::msh::{e_arg::*, e_redir::*, eprint_msh, t_arg};
use nix::{
	fcntl::{OFlag, open},
	sys::stat::Mode,
	unistd::{AccessFlags, access},
};
use std::ffi::CString;

pub(super) fn do_redirections(cmd_args: &mut [t_arg]) -> Result<(), i32> {
	let mut i = 0;
	while i < cmd_args.len() {
		if (cmd_args[i]).type_0 == REDIR && (cmd_args[i]).redir.unwrap() != HEREDOC {
			let file = CString::new((cmd_args[i]).elem_str.clone()).unwrap();
			let file = file.as_c_str();
			let fd_result = match (cmd_args[i]).redir.unwrap() {
				INPUT_REDIR => {
					if access(file, AccessFlags::F_OK).is_err() {
						return Err(127);
					} else if access(file, AccessFlags::R_OK).is_err() {
						return Err(126);
					}
					open(file, OFlag::O_RDONLY, Mode::empty())
				}
				OUTPUT_REDIR => open(
					file,
					OFlag::O_WRONLY | OFlag::O_CREAT | OFlag::O_TRUNC,
					Mode::from_bits(0o644).unwrap(),
				),
				APPEND => {
					if access(file, AccessFlags::W_OK).is_err() {
						return Err(126);
					}
					open(
						file,
						OFlag::O_WRONLY | OFlag::O_CREAT | OFlag::O_APPEND,
						Mode::from_bits(0o644).unwrap(),
					)
				}
				_ => unreachable!(),
			};
			match fd_result {
				Ok(fd) => {
					if (cmd_args[i]).redir != Some(INPUT_REDIR) {
						let _ = nix::unistd::dup2(fd, 1);
					} else {
						let _ = nix::unistd::dup2(fd, 0);
					}
					let _ = nix::unistd::close(fd);
				}
				Err(_) => {
					eprint_msh!("failed to execute: {}", (cmd_args[i]).elem_str);
					return Err(-1);
				}
			}
		}
		i += 1;
	}
	Ok(())
}
