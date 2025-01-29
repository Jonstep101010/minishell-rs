use crate::prelude::*;
use nix::{fcntl::OFlag, sys::stat::Mode};
use std::str::FromStr;

fn heredoc_loop(delim: &str, fd: i32, env: &Env) {
	// g_ctrl_c = 0;
	// not handling ctrl c signals for now
	loop {
		let opt_line = unsafe { crate::utils::rust_readline::str_readline("> ") };
		match opt_line {
			Some(line) if line != delim => {
				let expand_input = CString::from_str(&line).unwrap();
				if let Some(expanded) = crate::environment::expander::expander(&expand_input, env) {
					unsafe { libc::write(fd, expanded.as_ptr().cast(), expanded.count_bytes()) };
				}
			}
			_ => return,
		}
	}
}

pub fn do_heredocs(token: &t_token, target: &mut i32, env: &Env) {
	let mut i = 0;
	while i < token.cmd_args_vec.len() {
		if (token.cmd_args_vec[i]).redir == Some(HEREDOC) {
			let oflags = OFlag::O_RDWR | OFlag::O_CREAT | OFlag::O_TRUNC;
			let mode = Mode::from_bits(0o644).expect("Invalid mode");
			match nix::fcntl::open(c".heredoc.txt", oflags, mode) {
				Ok(mut fd) => {
					heredoc_loop(
						unsafe {
							CStr::from_ptr((token.cmd_args_vec[i]).elem)
								.to_str()
								.unwrap()
						},
						fd,
						env,
					);
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
