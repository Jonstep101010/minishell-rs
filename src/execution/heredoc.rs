use crate::prelude::*;
use nix::{fcntl::OFlag, sys::stat::Mode};
use std::os::fd::BorrowedFd;

fn heredoc_loop(delim: &str, fd: i32, env: &Env) {
	// g_ctrl_c = 0;
	// not handling ctrl c signals for now
	loop {
		let opt_line = crate::utils::rust_readline::str_readline("> ");
		match opt_line {
			Some(line) if line != delim => {
				let expanded = env.expander(&line);
				let mut output = expanded.into_bytes();
				output.push(b'\n');
				let safe_fd = unsafe { BorrowedFd::borrow_raw(fd) };
				if let Err(e) = nix::unistd::write(safe_fd, &output) {
					eprintln!("heredoc write error: {}", e);
					return;
				}
			}
			_ => return,
		}
	}
}

pub(super) fn do_heredocs(token: &t_token, target: &mut i32, env: &Env) {
	let mut i = 0;
	while i < token.cmd_args_vec.len() {
		if (token.cmd_args_vec[i]).redir == Some(HEREDOC) {
			let oflags = OFlag::O_RDWR | OFlag::O_CREAT | OFlag::O_TRUNC;
			let mode = Mode::from_bits(0o644).expect("Invalid mode");
			match nix::fcntl::open(c".heredoc.txt", oflags, mode) {
				Ok(mut fd) => {
					heredoc_loop(&(token.cmd_args_vec[i]).elem_str, fd, env);
					nix::unistd::close(fd).unwrap();
					fd = nix::fcntl::open(c".heredoc.txt", OFlag::O_RDONLY, Mode::empty()).unwrap();
					nix::unistd::dup2(fd, *target).unwrap();
					nix::unistd::close(fd).unwrap();
					let _ = nix::unistd::unlink(c".heredoc.txt");
				}
				Err(e) => {
					eprint_msh!("{}", e);
					panic!("i/o error");
				}
			}
		}
		i += 1;
	}
}
