mod bin_path;
mod exec_bin;
mod execute_pipes;
mod heredoc;
mod redirections;

pub mod builtins {
	pub mod cd;
	pub mod echo;
	pub mod env;
	pub mod exit;
	pub mod export;
	pub mod pwd;
	pub mod unset;
	pub use {cd::cd, echo::echo, env::env, exit::exit, export::export, pwd::pwd, unset::unset};
}

use crate::{
	execution::{exec_bin::exec_bin, execute_pipes::execute_pipes, redirections::do_redirections},
	msh::{ArgType::Redir, CommandArg, CommandToken, Env, ShellState, eprint_msh},
};
use std::ffi::CString;
use std::os::fd::OwnedFd;

fn backup_and_apply_parent_redirections(
	cmd_args: &mut [CommandArg],
) -> Result<(OwnedFd, OwnedFd), i32> {
	let stdin_bak = nix::unistd::dup(std::io::stdin()).map_err(|_| 1)?;
	let stdout_bak = nix::unistd::dup(std::io::stdout()).map_err(|_| 1)?;
	do_redirections(cmd_args)?;
	Ok((stdin_bak, stdout_bak))
}

fn restore_parent_stdio(stdin_bak: &OwnedFd, stdout_bak: &OwnedFd) -> Result<(), i32> {
	let restore_stdin = nix::unistd::dup2_stdin(stdin_bak);
	let restore_stdout = nix::unistd::dup2_stdout(stdout_bak);
	if restore_stdin.is_err() || restore_stdout.is_err() {
		return Err(1);
	}
	Ok(())
}

impl crate::CommandToken {
	pub fn get_args_vec(&self) -> Vec<CString> {
		// assert!(
		// 	!self.cmd_args_vec.is_empty() && !self.cmd_args_vec[0].elem_str.is_empty(),
		// 	"token has to contain something"
		// );
		let mut vec_cstr = Vec::new();
		vec_cstr.extend(
			self.cmd_args_vec
				.iter()
				.filter(|arg| arg.type_0.is_none_or(|redir| redir != Redir))
				.map(|arg| CString::new(arg.elem_str.clone()).unwrap()),
		);
		debug_assert!(!vec_cstr.is_empty());
		vec_cstr
	}
}

pub(crate) fn execute_commands(shell: &mut ShellState) {
	match shell.token_len.unwrap() {
		0 => unreachable!("there should not be empty tokens here"),
		1 if shell.token_vec[0].has_redir && shell.token_vec[0].cmd_name.is_empty() => {
			// redirection only (no cmd name):
			// < infile
			// > outfile
			// >> outfile
			// < infile > outfile
			// >> out < in
			assert!(
				!shell.token_vec[0].cmd_args_vec.is_empty()
					&& shell.token_vec[0].cmd_args_vec[0].redir.is_some(),
				"redirs not found"
			);
			match backup_and_apply_parent_redirections(&mut shell.token_vec[0].cmd_args_vec) {
				Ok((stdin_bak, stdout_bak)) => {
					shell.env.set_status(0);
					if let Err(status) = restore_parent_stdio(&stdin_bak, &stdout_bak) {
						eprint_msh!("failed to restore stdio");
						shell.env.set_status(status);
					}
				}
				Err(status) => {
					eprint_msh!("failed to do redirections");
					shell.env.set_status(status);
				}
			}
		}
		1 if !{
			shell.token_vec[0].cmd_name != b"cd"
				&& shell.token_vec[0].cmd_name != b"unset"
				&& shell.token_vec[0].cmd_name != b"export"
				&& shell.token_vec[0].cmd_name != b"exit"
		} =>
		{
			match backup_and_apply_parent_redirections(&mut shell.token_vec[0].cmd_args_vec) {
				Ok((stdin_bak, stdout_bak)) => {
					executor(&mut shell.token_vec[0], &mut shell.env);
					if let Err(status) = restore_parent_stdio(&stdin_bak, &stdout_bak) {
						eprint_msh!("failed to restore stdio");
						shell.env.set_status(status);
					}
				}
				Err(status) => {
					eprint_msh!("failed to do redirections");
					shell.env.set_status(status);
				}
			}
		}
		_ => {
			execute_pipes(shell);
		}
	}
	shell.restore();
}

fn executor(token: &mut CommandToken, shell_env: &mut Env) {
	let args = token.get_args_vec();
	let status = match token.cmd_name.as_slice() {
		b"echo" => builtins::echo(&args),
		b"cd" => builtins::cd(shell_env, &args),
		b"pwd" => builtins::pwd(shell_env),
		b"export" => builtins::export(shell_env, &args),
		b"unset" => builtins::unset(shell_env, &args),
		b"env" => builtins::env(shell_env),
		b"exit" => builtins::exit(shell_env, &args),
		_ => exec_bin(shell_env, &args),
	};
	shell_env.set_status(status);
}
