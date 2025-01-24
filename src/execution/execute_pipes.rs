use crate::t_shell;
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
	if cpid == 0 {
		// check_signals_child(&mut (*shell).p_termios);
		if (*(shell.token).add(i)).has_redir {
			do_heredocs(&mut *(shell.token).add(i), prevpipe, &shell.env);
		}
		if do_redirections((*(shell.token).add(i)).cmd_args, error_elem) != 0 {
			if !error_elem.is_null() {
				todo!("display error");
				// eprint_msh!("{}: {}", error_elem, error);
			}
			crate::tokenizer::destroy_tokens::destroy_all_tokens(&mut (*shell));
			// free(shell as *mut libc::c_void);
			todo!("bail out gracefully");
		}
		dup2(*prevpipe, 0);
		close(*prevpipe);
		let ret = ((*(shell.token).add(i)).cmd_func).expect("non-null function pointer")(
			shell,
			&mut *(shell.token).add(i),
		);
		crate::tokenizer::destroy_tokens::destroy_all_tokens(&mut (*shell));
		// free(shell as *mut libc::c_void);
		std::process::exit(ret);
	} else {
		waitpid(cpid, &mut status, 0);
		close(*prevpipe);
		while wait(std::ptr::null_mut::<i32>()) > 0 {}
		if status & 0x7f == 0 {
			shell.exit_status = ((status & 0xff00) >> 8) as u8;
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
	if cpid == 0 {
		// check_signals_child(&mut (*shell).p_termios);
		close(pipefd[0_usize]);
		dup2(pipefd[1_usize], 1);
		close(pipefd[1_usize]);
		dup2(*prevpipe, 0);
		close(*prevpipe);
		if do_redirections((*(shell.token).add(i)).cmd_args, error_elem) != 0 {
			if !error_elem.is_null() {
				todo!("display error");
				// eprint_msh!("{}: {}", error_elem, error);
			}
			crate::tokenizer::destroy_tokens::destroy_all_tokens(&mut (*shell));
			// free(shell as *mut libc::c_void);
			todo!("bail out gracefully");
		}
		let ret = ((*(shell.token).add(i)).cmd_func).expect("non-null function pointer")(
			shell,
			&mut *(shell.token).add(i),
		);
		{
			let mut shell: &mut t_shell = shell;
			let mut exit_code = ret;
			crate::tokenizer::destroy_tokens::destroy_all_tokens(&mut (*shell));
			// free(shell as *mut libc::c_void);
			std::process::exit(exit_code);
		};
	} else {
		close(pipefd[1_usize]);
		close(*prevpipe);
		*prevpipe = pipefd[0_usize];
	};
}
#[unsafe(no_mangle)]
pub unsafe fn execute_pipes(mut shell: &mut t_shell) {
	let mut i = 0;
	let mut error_elem: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
	let mut prevpipe = dup(0);
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
