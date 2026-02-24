#![allow(unsafe_op_in_unsafe_fn)]

use std::os::fd::{AsFd, AsRawFd, OwnedFd};

use super::{executor, heredoc::do_heredocs, redirections::do_redirections};
use crate::t_shell;
use nix::{
	errno::Errno,
	sys::wait::{WaitStatus, waitpid},
	unistd::{ForkResult, Pid},
};
use std::io::stdin;
fn exec_last(shell: &mut t_shell, i: usize, mut prevpipe: OwnedFd, pids: &mut Vec<Pid>) {
	match unsafe { nix::unistd::fork() } {
		Ok(ForkResult::Parent { child }) => {
			pids.push(child);
			drop(prevpipe);
			let mut last_status = shell.env.get_status();
			let mut remaining = pids.len();
			while remaining > 0 {
				match waitpid(None, None) {
					Ok(WaitStatus::Exited(waited_pid, exit_code)) => {
						if waited_pid == child {
							last_status = exit_code;
						}
						remaining -= 1;
					}
					Ok(WaitStatus::Signaled(waited_pid, signal, _)) => {
						if waited_pid == child {
							last_status = 128 + signal as i32;
						}
						remaining -= 1;
					}
					Ok(_) => remaining -= 1,
					Err(Errno::EINTR) => continue,
					Err(Errno::ECHILD) => break,
					Err(e) => {
						eprintln!("waitpid failed: {}", e);
						break;
					}
				}
			}
			shell.env.set_status(last_status);
		}
		Ok(ForkResult::Child) => {
			// Restore default SIGPIPE behavior so commands die silently on closed pipes
			unsafe {
				nix::sys::signal::signal(
					nix::sys::signal::Signal::SIGPIPE,
					nix::sys::signal::SigHandler::SigDfl,
				)
				.expect("pipe dfl handler");
			}
			if shell.token_vec[i].has_redir {
				do_heredocs(&shell.token_vec[i], &mut prevpipe, &shell.env);
			}
			if do_redirections(&mut shell.token_vec[i].cmd_args_vec).is_err() {
				panic!("failed to do redirections");
			}
			nix::unistd::dup2_stdin(&prevpipe).expect("dup2 stdin failed");
			drop(prevpipe);
			executor(&mut shell.token_vec[i], &mut shell.env);
			std::process::exit(shell.env.get_status());
		}
		Err(e) => eprintln!("fork failed: {}", e),
	}
}

fn exec_pipe(shell: &mut t_shell, i: usize, prevpipe: &mut OwnedFd, pids: &mut Vec<Pid>) {
	let pipefd = nix::unistd::pipe().expect("pipe fail");
	match unsafe { nix::unistd::fork() } {
		Ok(ForkResult::Parent { child }) => {
			pids.push(child);
			drop(pipefd.1);
			let old_prevpipe = std::mem::replace(prevpipe, pipefd.0);
			drop(old_prevpipe);
		}
		Ok(ForkResult::Child) => {
			// Restore default SIGPIPE behavior so commands die silently on closed pipes
			unsafe {
				nix::sys::signal::signal(
					nix::sys::signal::Signal::SIGPIPE,
					nix::sys::signal::SigHandler::SigDfl,
				)
				.expect("pipe dfl handler");
			}
			drop(pipefd.0);
			nix::unistd::dup2_stdout(&pipefd.1).expect("dup2 stdout failed");
			drop(pipefd.1);
			nix::unistd::dup2_stdin(prevpipe.as_fd()).expect("dup2 stdin failed");
			nix::unistd::close(prevpipe.as_raw_fd()).expect("close prevpipe after dup2");
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
	let mut prevpipe = nix::unistd::dup(stdin()).unwrap();
	let mut pids = Vec::with_capacity(shell.token_vec.len());
	for i in 0..shell.token_len.unwrap() - 1 {
		if shell.token_vec[i].has_redir && i != shell.token_len.unwrap() - 1 {
			do_heredocs(&shell.token_vec[i], &mut prevpipe, &shell.env);
		}
		exec_pipe(shell, i, &mut prevpipe, &mut pids);
	}
	exec_last(shell, shell.token_len.unwrap() - 1, prevpipe, &mut pids);
}
