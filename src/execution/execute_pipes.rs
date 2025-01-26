use crate::t_shell;
use ::libc;
use libc::{close, dup, dup2, fork, pipe, wait, waitpid};

use super::{executor, heredoc::do_heredocs, redirections::do_redirections};

unsafe fn exec_last(shell: &mut t_shell, i: usize, prevpipe: *mut i32) {
	let mut status: i32 = 0;
	let cpid = fork();
	if cpid == 0 {
		// check_signals_child(&mut (*shell).p_termios);
		if (*(shell.token).add(i)).has_redir {
			do_heredocs(&*(shell.token).add(i), prevpipe, &shell.env);
		}
		if do_redirections(&mut (*(shell.token).add(i)).cmd_args_vec).is_err() {
			crate::tokenizer::destroy_tokens::destroy_all_tokens(&mut (*shell));
			todo!("bail out gracefully");
		}
		dup2(*prevpipe, 0);
		close(*prevpipe);
		let ret = executor((shell.token).add(i), shell);
		crate::tokenizer::destroy_tokens::destroy_all_tokens(&mut (*shell));
		std::process::exit(ret);
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
		if do_redirections(&mut (*(shell.token).add(i)).cmd_args_vec).is_err() {
			crate::tokenizer::destroy_tokens::destroy_all_tokens(&mut (*shell));
			todo!("bail out gracefully");
		}
		let ret = executor((shell.token).add(i), shell);
		crate::tokenizer::destroy_tokens::destroy_all_tokens(&mut (*shell));
		std::process::exit(ret);
	} else {
		close(pipefd[1_usize]);
		close(*prevpipe);
		*prevpipe = pipefd[0_usize];
	};
}
#[unsafe(no_mangle)]
pub unsafe fn execute_pipes(shell: &mut t_shell) {
	let mut prevpipe = dup(0);
	for i in 0..shell.token_len.unwrap() - 1 {
		if (*(shell.token).add(i)).has_redir && i != shell.token_len.unwrap() - 1 {
			do_heredocs(&*(shell.token).add(i), &mut prevpipe, &shell.env);
		}
		exec_pipe(shell, i, &mut prevpipe);
	}
	exec_last(shell, shell.token_len.unwrap() - 1, &mut prevpipe);
}
