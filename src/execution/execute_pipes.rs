use ::libc;
extern "C" {
	fn update_exit_status(shell: *mut t_shell, status: libc::c_int);
	fn exit_free(shell: *mut t_shell, exit_code: libc::c_int);
	fn exit_error(shell: *mut t_shell, error_elem: *mut libc::c_char);
	fn do_redirections(cmd_args: *mut t_arg, error_elem: *mut *mut libc::c_char) -> libc::c_int;
	fn do_heredocs(token: *mut t_token, target: *const libc::c_int, env: *mut *mut libc::c_char);
	fn wait(__stat_loc: *mut libc::c_int) -> __pid_t;
	fn waitpid(__pid: __pid_t, __stat_loc: *mut libc::c_int, __options: libc::c_int) -> __pid_t;
	fn close(__fd: libc::c_int) -> libc::c_int;
	fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
	fn dup(__fd: libc::c_int) -> libc::c_int;
	fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
	fn fork() -> __pid_t;
	fn check_signals_child(p_termios_child: *mut termios);
}
pub type size_t = libc::c_ulong;
use crate::t_shell;
pub type speed_t = libc::c_uint;
pub type cc_t = libc::c_uchar;
pub type tcflag_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type __uint8_t = libc::c_uchar;
use crate::t_arg;
use crate::t_token;
use crate::termios;
pub type e_redir = libc::c_uint;
pub const HEREDOC: e_redir = 4;
pub const APPEND: e_redir = 3;
pub const OUTPUT_REDIR: e_redir = 2;
pub const INPUT_REDIR: e_redir = 1;
pub const NO_REDIR: e_redir = 0;
pub type e_arg = libc::c_uint;
pub const REDIR_REMOVED: e_arg = 2;
pub const REDIR: e_arg = 1;
pub const STRING: e_arg = 0;
pub type __pid_t = libc::c_int;
pub type pid_t = __pid_t;
unsafe extern "C" fn exec_last(
	mut shell: *mut t_shell,
	mut i: libc::c_int,
	mut prevpipe: *mut libc::c_int,
	mut error_elem: *mut *mut libc::c_char,
) {
	let mut cpid: pid_t = 0;
	let mut status: libc::c_int = 0;
	cpid = fork();
	status = 0 as libc::c_int;
	if cpid == 0 as libc::c_int {
		check_signals_child(&mut (*shell).p_termios);
		if (*((*shell).token).offset(i as isize)).has_redir {
			do_heredocs(
				&mut *((*shell).token).offset(i as isize),
				prevpipe,
				(*shell).env,
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
		while wait(0 as *mut libc::c_int) > 0 as libc::c_int {}
		if status & 0x7f as libc::c_int == 0 as libc::c_int {
			update_exit_status(shell, (status & 0xff00 as libc::c_int) >> 8 as libc::c_int);
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
	let mut cpid: pid_t = 0;
	pipe(pipefd.as_mut_ptr());
	cpid = fork();
	if cpid == 0 as libc::c_int {
		check_signals_child(&mut (*shell).p_termios);
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
#[no_mangle]
pub unsafe extern "C" fn execute_pipes(mut shell: *mut t_shell, mut token_count: libc::c_int) {
	let mut i: libc::c_int = 0;
	let mut prevpipe: libc::c_int = 0;
	let mut error_elem: *mut libc::c_char = 0 as *mut libc::c_char;
	i = -(1 as libc::c_int);
	prevpipe = dup(0 as libc::c_int);
	loop {
		i += 1;
		if !(i < token_count - 1 as libc::c_int) {
			break;
		}
		if (*((*shell).token).offset(i as isize)).has_redir as libc::c_int != 0
			&& i != token_count - 1 as libc::c_int
		{
			do_heredocs(
				&mut *((*shell).token).offset(i as isize),
				&mut prevpipe,
				(*shell).env,
			);
		}
		exec_pipe(shell, i, &mut prevpipe, &mut error_elem);
	}
	exec_last(shell, i, &mut prevpipe, &mut error_elem);
}
