#![allow(unsafe_op_in_unsafe_fn)]

use super::{executor, heredoc::do_heredocs, redirections::do_redirections};
use crate::t_shell;
use ::libc;
use nix::{
	sys::wait::{WaitStatus, waitpid},
	unistd::{ForkResult, fork},
};

unsafe fn exec_last(shell: &mut t_shell, i: usize, prevpipe: *mut i32) {
	match unsafe { fork() } {
		Ok(ForkResult::Parent { child }) => match waitpid(child, None) {
			Ok(WaitStatus::Exited(_, exit_code)) => {
				shell.env.set_status(exit_code);
			}
			Ok(WaitStatus::Signaled(_, signal, _)) => {
				shell.env.set_status(128 + signal as i32);
			}
			Err(e) => eprintln!("waitpid failed: {}", e),
			_ => (),
		},
		Ok(ForkResult::Child) => {
			// previously: check signals child
			if shell.token_vec[i].has_redir {
				do_heredocs(&shell.token_vec[i], &mut *prevpipe, &shell.env);
			}
			if do_redirections(&mut shell.token_vec[i].cmd_args_vec).is_err() {
				panic!("failed to do redirections");
			}
			libc::dup2(*prevpipe, 0);
			libc::close(*prevpipe);
			executor(&mut shell.token_vec[i], &mut shell.env);
			std::process::exit(shell.env.get_status());
		}
		Err(e) => eprintln!("fork failed: {}", e),
	}
}

unsafe fn exec_pipe(shell: &mut t_shell, i: usize, prevpipe: *mut i32) {
	let mut pipefd: [i32; 2] = [0; 2];
	libc::pipe(pipefd.as_mut_ptr());
	match unsafe { fork() } {
		Ok(ForkResult::Parent { .. }) => {
			libc::close(pipefd[1_usize]);
			libc::close(*prevpipe);
			*prevpipe = pipefd[0_usize];
		}
		Ok(ForkResult::Child) => {
			// previously: check signals child
			libc::close(pipefd[0_usize]);
			libc::dup2(pipefd[1_usize], 1);
			libc::close(pipefd[1_usize]);
			libc::dup2(*prevpipe, 0);
			libc::close(*prevpipe);
			if do_redirections(&mut shell.token_vec[i].cmd_args_vec).is_err() {
				panic!("failed to do redirections");
			}
			executor(&mut shell.token_vec[i], &mut shell.env);
			std::process::exit(shell.env.get_status());
		}
		Err(e) => eprintln!("fork failed: {}", e),
	};
}

pub(super) fn execute_pipes(shell: &mut t_shell) {
	let mut prevpipe = nix::unistd::dup(0).unwrap();
	for i in 0..shell.token_len.unwrap() - 1 {
		if shell.token_vec[i].has_redir && i != shell.token_len.unwrap() - 1 {
			do_heredocs(&shell.token_vec[i], &mut prevpipe, &shell.env);
		}
		unsafe {
			exec_pipe(shell, i, &mut prevpipe);
		}
	}
	unsafe {
		exec_last(shell, shell.token_len.unwrap() - 1, &mut prevpipe);
	}
}
