use crate::msh::{CommandToken, Env, RedirType::HereDoc, eprint_msh};
use nix::{fcntl::OFlag, sys::stat::Mode};
use std::os::fd::OwnedFd;

pub(super) fn do_heredocs(token: &CommandToken, target: &mut OwnedFd, env: &Env) {
	let mut i = 0;
	while i < token.cmd_args_vec.len() {
		if (token.cmd_args_vec[i]).redir == Some(HereDoc) {
			let oflags = OFlag::O_RDWR | OFlag::O_CREAT | OFlag::O_TRUNC;
			let mode = Mode::from_bits(0o644).expect("Invalid mode");
			let mut rl = rustyline::DefaultEditor::new().unwrap();
			match nix::fcntl::open(c".heredoc.txt", oflags, mode) {
				Ok(mut fd) => {
					let delim: &str = &(token.cmd_args_vec[i]).elem_str;
					loop {
						match rl.readline("> ") {
							Ok(mut line) if line != delim => {
								env.expander(&mut line);
								let mut output = line.into_bytes();
								output.push(b'\n');
								if let Err(e) = nix::unistd::write(&fd, &output) {
									eprintln!("heredoc write error: {e}");
									break;
								}
							}
							Err(rustyline::error::ReadlineError::Eof) => {} // continue;
							_ => break,
						}
					}
					nix::unistd::close(fd).unwrap();
					fd = nix::fcntl::open(c".heredoc.txt", OFlag::O_RDONLY, Mode::empty()).unwrap();
					nix::unistd::dup2(&fd, target).unwrap();
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
