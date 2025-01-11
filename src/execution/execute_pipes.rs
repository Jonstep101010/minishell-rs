use crate::{
	t_shell,
	utils::exit_free::{exit_error, exit_free},
};
use ::libc;
use libc::{close, dup, dup2, fork, pipe, wait, waitpid};

use super::{heredoc::do_heredocs, redirections::do_redirections};

unsafe extern "C" fn exec_last(
	mut shell: *mut t_shell,
	mut i: libc::c_int,
	mut prevpipe: *mut libc::c_int,
	mut error_elem: *mut *mut libc::c_char,
) {
	let mut status: libc::c_int = 0;
	let mut cpid = fork();
	if cpid == 0 as libc::c_int {
		// check_signals_child(&mut (*shell).p_termios);
		if (*((*shell).token).offset(i as isize)).has_redir {
			do_heredocs(
				&mut *((*shell).token).offset(i as isize),
				prevpipe,
				&(*shell).env,
			);
		}
		if do_redirections((*((*shell).token).offset(i as isize)).cmd_args, error_elem)
			!= 0 as libc::c_int
		{
			exit_error(shell, *error_elem);
		}
		dup2(*prevpipe, 0 as libc::c_int);
		close(*prevpipe);
		exit_free(
			shell,
			((*((*shell).token).offset(i as isize)).cmd_func).expect("non-null function pointer")(
				shell,
				&mut *((*shell).token).offset(i as isize),
			),
		);
	} else {
		waitpid(cpid, &mut status, 0 as libc::c_int);
		close(*prevpipe);
		while wait(std::ptr::null_mut::<libc::c_int>()) > 0 as libc::c_int {}
		if status & 0x7f as libc::c_int == 0 as libc::c_int {
			(*shell).exit_status = ((status & 0xff00 as libc::c_int) >> 8 as libc::c_int) as u8;
		}
	};
}
unsafe extern "C" fn exec_pipe(
	mut shell: *mut t_shell,
	mut i: libc::c_int,
	mut prevpipe: *mut libc::c_int,
	mut error_elem: *mut *mut libc::c_char,
) {
	let mut pipefd: [libc::c_int; 2] = [0; 2];
	pipe(pipefd.as_mut_ptr());
	let mut cpid = fork();
	if cpid == 0 as libc::c_int {
		// check_signals_child(&mut (*shell).p_termios);
		close(pipefd[0 as libc::c_int as usize]);
		dup2(pipefd[1 as libc::c_int as usize], 1 as libc::c_int);
		close(pipefd[1 as libc::c_int as usize]);
		dup2(*prevpipe, 0 as libc::c_int);
		close(*prevpipe);
		if do_redirections((*((*shell).token).offset(i as isize)).cmd_args, error_elem)
			!= 0 as libc::c_int
		{
			exit_error(shell, *error_elem);
		}
		exit_free(
			shell,
			((*((*shell).token).offset(i as isize)).cmd_func).expect("non-null function pointer")(
				shell,
				&mut *((*shell).token).offset(i as isize),
			),
		);
	} else {
		close(pipefd[1 as libc::c_int as usize]);
		close(*prevpipe);
		*prevpipe = pipefd[0 as libc::c_int as usize];
	};
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn execute_pipes(mut shell: *mut t_shell, mut token_count: libc::c_int) {
	let mut i: libc::c_int = -1;
	let mut error_elem: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
	let mut prevpipe = dup(0 as libc::c_int);
	loop {
		i += 1;
		if i >= token_count - 1 as libc::c_int {
			break;
		}
		if (*((*shell).token).offset(i as isize)).has_redir as libc::c_int != 0
			&& i != token_count - 1 as libc::c_int
		{
			do_heredocs(
				&mut *((*shell).token).offset(i as isize),
				&mut prevpipe,
				&(*shell).env,
			);
		}
		exec_pipe(shell, i, &mut prevpipe, &mut error_elem);
	}
	exec_last(shell, i, &mut prevpipe, &mut error_elem);
}
