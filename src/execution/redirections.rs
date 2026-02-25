use crate::msh::{
	ArgType::Redir,
	CommandArg,
	RedirType::{Append, HereDoc, InputRedir, OutputRedir},
	eprint_msh,
};
use nix::{
	fcntl::{OFlag, open},
	sys::stat::Mode,
	unistd::{AccessFlags, access},
};
use std::ffi::CString;

pub(super) fn do_redirections(cmd_args: &mut [CommandArg]) -> Result<(), i32> {
	let mut i = 0;
	while i < cmd_args.len() {
		if (cmd_args[i]).type_0 == Some(Redir) && (cmd_args[i]).redir.unwrap() != HereDoc {
			let file = CString::new((cmd_args[i]).elem_str.clone()).unwrap();
			let file = file.as_c_str();
			let fd_result = match (cmd_args[i]).redir.unwrap() {
				InputRedir => {
					if access(file, AccessFlags::F_OK).is_err() {
						return Err(127);
					} else if access(file, AccessFlags::R_OK).is_err() {
						return Err(126);
					}
					open(file, OFlag::O_RDONLY, Mode::empty())
				}
				OutputRedir => open(
					file,
					OFlag::O_WRONLY | OFlag::O_CREAT | OFlag::O_TRUNC,
					Mode::from_bits(0o644).unwrap(),
				),
				Append => {
					if access(file, AccessFlags::W_OK).is_err() {
						return Err(126);
					}
					open(
						file,
						OFlag::O_WRONLY | OFlag::O_CREAT | OFlag::O_APPEND,
						Mode::from_bits(0o644).unwrap(),
					)
				}
				HereDoc => unreachable!(),
			};
			if let Ok(fd) = fd_result {
				if (cmd_args[i]).redir == Some(InputRedir) {
					let _ = nix::unistd::dup2_stdin(&fd);
				} else {
					let _ = nix::unistd::dup2_stdout(&fd);
				}
				let _ = nix::unistd::close(fd);
			} else {
				eprint_msh!("failed to execute: {}", (cmd_args[i]).elem_str);
				return Err(-1);
			}
		}
		i += 1;
	}
	Ok(())
}
