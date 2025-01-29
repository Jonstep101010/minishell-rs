#![allow(unsafe_op_in_unsafe_fn)]
use ::libc;
use libc::{close, dup2, fork, pipe, wait, waitpid};

use super::{eprint_msh, executor, heredoc::do_heredocs, redirections::do_redirections, t_shell};

unsafe fn exec_last(shell: &mut t_shell, i: usize, prevpipe: *mut i32) {
	let mut status: i32 = 0;
	let cpid = fork();
	if cpid == 0 {
		// check_signals_child(&mut (*shell).p_termios);
		if shell.token_vec[i].has_redir {
			do_heredocs(&shell.token_vec[i], &mut *prevpipe, &shell.env);
		}
		if do_redirections(&mut shell.token_vec[i].cmd_args_vec).is_err() {
			shell.restore();
			eprint_msh!("failed to do redirections");
			std::process::exit(1);
		}
		dup2(*prevpipe, 0);
		close(*prevpipe);
		executor(&mut shell.token_vec[i], &mut shell.env);
		std::process::exit(shell.env.get_status());
	} else {
		waitpid(cpid, &mut status, 0);
		close(*prevpipe);
		while wait(std::ptr::null_mut::<i32>()) > 0 {}
		if status & 0x7f == 0 {
			shell.env.set_status(((status & 0xff00) >> 8) as u8 as i32);
		}
	};
}

unsafe fn exec_pipe(shell: &mut t_shell, i: usize, prevpipe: *mut i32) {
	let mut pipefd: [i32; 2] = [0; 2];
	pipe(pipefd.as_mut_ptr());
	let cpid = fork();
	if cpid == 0 {
		// check_signals_child(&mut (*shell).p_termios);
		close(pipefd[0_usize]);
		dup2(pipefd[1_usize], 1);
		close(pipefd[1_usize]);
		dup2(*prevpipe, 0);
		close(*prevpipe);
		if let Err(status) = do_redirections(&mut shell.token_vec[i].cmd_args_vec) {
			eprint_msh!("failed to do redirections");
			std::process::exit(status);
		}
		executor(&mut shell.token_vec[i], &mut shell.env);
		std::process::exit(shell.env.get_status());
	} else {
		close(pipefd[1_usize]);
		close(*prevpipe);
		*prevpipe = pipefd[0_usize];
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
