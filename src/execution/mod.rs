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
	msh::{Env, e_arg::*, eprint_msh, t_shell, t_token},
};
use std::ffi::CString;

impl crate::t_token {
	pub fn get_args_vec(&self) -> Vec<CString> {
		assert!(
			!self.cmd_args_vec.is_empty() && !self.cmd_args_vec[0].elem_str.is_empty(),
			"token has to contain something"
		);
		let mut vec_cstr = Vec::new();
		vec_cstr.extend(
			self.cmd_args_vec
				.iter()
				.filter(|arg| arg.type_0 != REDIR)
				.map(|arg| CString::new(arg.elem_str.clone()).unwrap()),
		);
		debug_assert!(!vec_cstr.is_empty());
		vec_cstr
	}
}

pub(crate) fn execute_commands(shell: &mut t_shell) {
	match shell.token_len.unwrap() {
		0 => unreachable!("there should not be empty tokens here"),
		1 if !{
			shell.token_vec[0].cmd_name != b"cd"
				&& shell.token_vec[0].cmd_name != b"unset"
				&& shell.token_vec[0].cmd_name != b"export"
				&& shell.token_vec[0].cmd_name != b"exit"
		} =>
		{
			if let Err(status) = do_redirections(&mut shell.token_vec[0].cmd_args_vec) {
				eprint_msh!("failed to do redirections");
				std::process::exit(status)
			}
			executor(&mut shell.token_vec[0], &mut shell.env)
		}
		_ => {
			execute_pipes(shell);
		}
	}
	shell.restore();
}

fn executor(token: &mut t_token, shell_env: &mut Env) {
	let args = token.get_args_vec();
	let status = match token.cmd_name.as_slice() {
		b"echo" => builtins::echo(args),
		b"cd" => builtins::cd(shell_env, args),
		b"pwd" => builtins::pwd(shell_env),
		b"export" => builtins::export(shell_env, args),
		b"unset" => builtins::unset(shell_env, args),
		b"env" => builtins::env(shell_env),
		b"exit" => builtins::exit(shell_env, args),
		_ => exec_bin(shell_env, &args),
	};
	shell_env.set_status(status);
}
