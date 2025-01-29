mod bin_path;
mod exec_bin;
mod execute_pipes;
mod heredoc;
mod redirections;

use crate::{
	execution::{execute_pipes::execute_pipes, redirections::do_redirections},
	prelude::*,
};

use exec_bin::exec_bin;
pub fn execute_commands(shell: &mut t_shell) {
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
pub mod builtins;
pub use builtins::{cd, echo, env, exit, export, pwd, unset};
pub fn executor(token: &mut t_token, shell_env: &mut Env) {
	let args = crate::t_token::get_vec_cstr_token(token);
	assert!(!args.is_empty());
	let status = match token.cmd_name.as_slice() {
		b"echo" => echo(args),
		b"cd" => cd(shell_env, args),
		b"pwd" => pwd(shell_env),
		b"export" => export(shell_env, args),
		b"unset" => unset(shell_env, args),
		b"env" => env(shell_env),
		b"exit" => exit(shell_env, args),
		_ => exec_bin(shell_env, &args),
	};
	shell_env.set_status(status);
}
