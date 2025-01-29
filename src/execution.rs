mod bin_path;
pub(crate) mod exec_bin;
mod execute_pipes;
mod heredoc;
mod redirections;
use self::{execute_pipes::execute_pipes, redirections::do_redirections};
use crate::builtins::{
	cd::builtin_cd, echo::echo, env::builtin_env, exit::builtin_exit, export::builtin_export,
	pwd::builtin_pwd, unset::builtin_unset,
};
use crate::prelude::*;
use crate::tokenizer::build_command::get_vec_cstr_token;
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

pub fn executor(token: &mut t_token, shell_env: &mut Env) {
	let args = get_vec_cstr_token(token);
	assert!(!args.is_empty());
	let status = match token.cmd_name.as_slice() {
		b"echo" => echo(args),
		b"cd" => builtin_cd(shell_env, args),
		b"pwd" => builtin_pwd(shell_env),
		b"export" => builtin_export(shell_env, args),
		b"unset" => builtin_unset(shell_env, args),
		b"env" => builtin_env(shell_env),
		b"exit" => builtin_exit(shell_env, args),
		_ => exec_bin(shell_env, &args),
	};
	shell_env.set_status(status);
}
