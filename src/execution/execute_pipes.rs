use crate::{
	t_shell,
	utils::exit_free::{exit_error, exit_free},
};
use ::libc;
use libc::{close, dup, dup2, fork, pipe, wait, waitpid};

use super::{heredoc::do_heredocs, redirections::do_redirections};

unsafe fn exec_last(
	mut shell: &mut t_shell,
	mut i: usize,
	mut prevpipe: *mut i32,
	mut error_elem: *mut *mut libc::c_char,
) {
	let mut status: i32 = 0;
	let mut cpid = fork();
	if cpid == 0_i32 {
		// check_signals_child(&mut (*shell).p_termios);
		if (*(shell.token).add(i)).has_redir {
			do_heredocs(&mut *(shell.token).add(i), prevpipe, &shell.env);
		}
		if do_redirections((*(shell.token).add(i)).cmd_args, error_elem) != 0_i32 {
			exit_error(shell, *error_elem);
		}
		dup2(*prevpipe, 0_i32);
		close(*prevpipe);
		let ret = ((*(shell.token).add(i)).cmd_func).expect("non-null function pointer")(
			shell,
			&mut *(shell.token).add(i),
		);
		exit_free(shell, ret);
	} else {
		waitpid(cpid, &mut status, 0_i32);
		close(*prevpipe);
		while wait(std::ptr::null_mut::<i32>()) > 0_i32 {}
		if status & 0x7f_i32 == 0_i32 {
			shell.exit_status = ((status & 0xff00_i32) >> 8_i32) as u8;
		}
	};
}
unsafe fn exec_pipe(
	mut shell: &mut t_shell,
	mut i: usize,
	mut prevpipe: *mut i32,
	mut error_elem: *mut *mut libc::c_char,
) {
	let mut pipefd: [i32; 2] = [0; 2];
	pipe(pipefd.as_mut_ptr());
	let mut cpid = fork();
	if cpid == 0_i32 {
		// check_signals_child(&mut (*shell).p_termios);
		close(pipefd[0_i32 as usize]);
		dup2(pipefd[1_i32 as usize], 1_i32);
		close(pipefd[1_i32 as usize]);
		dup2(*prevpipe, 0_i32);
		close(*prevpipe);
		if do_redirections((*(shell.token).add(i)).cmd_args, error_elem) != 0_i32 {
			exit_error(shell, *error_elem);
		}
		let ret = ((*(shell.token).add(i)).cmd_func).expect("non-null function pointer")(
			shell,
			&mut *(shell.token).add(i),
		);
		exit_free(shell, ret);
	} else {
		close(pipefd[1_i32 as usize]);
		close(*prevpipe);
		*prevpipe = pipefd[0_i32 as usize];
	};
}
#[unsafe(no_mangle)]
pub unsafe fn execute_pipes(mut shell: &mut t_shell) {
	let mut i = 0;
	let mut error_elem: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
	let mut prevpipe = dup(0_i32);
	let token_count = shell.token_len.unwrap();
	loop {
		if i >= token_count - 1 {
			break;
		}
		if (*(shell.token).add(i)).has_redir as i32 != 0 && i != token_count - 1 {
			do_heredocs(&mut *(shell.token).add(i), &mut prevpipe, &shell.env);
		}
		exec_pipe(shell, i, &mut prevpipe, &mut error_elem);
		i += 1;
	}
	exec_last(shell, i, &mut prevpipe, &mut error_elem);
}
