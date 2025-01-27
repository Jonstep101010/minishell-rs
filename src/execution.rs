mod bin_path;
pub(crate) mod exec_bin;
mod execute_pipes;
mod heredoc;
mod redirections;
use self::{execute_pipes::execute_pipes, redirections::do_redirections};
use crate::{
	builtins::{
		cd::builtin_cd, echo::echo, env::builtin_env, exit::builtin_exit, export::builtin_export,
		pwd::builtin_pwd, unset::builtin_unset,
	},
	t_shell, t_token,
	tokenizer::destroy_tokens::destroy_all_tokens,
};
use exec_bin::exec_bin;

#[unsafe(no_mangle)]
pub unsafe fn execute_commands(shell: &mut t_shell) {
	let token = shell.token;
	match shell.token_len.unwrap() {
		0 => unreachable!("there should not be empty tokens here"),
		1 if !{
			(*token).cmd_name != b"cd"
				&& (*token).cmd_name != b"unset"
				&& (*token).cmd_name != b"export"
				&& (*token).cmd_name != b"exit"
		} =>
		{
			if do_redirections(&mut ((*token).cmd_args_vec)).is_err() {
				todo!("some sort of handling");
			}
			executor(&mut *token, shell);
		}
		_ => {
			execute_pipes(shell);
		}
	}
	destroy_all_tokens(&mut (*shell));
}

pub fn executor(token: &mut t_token, shell: &mut t_shell) {
	let args = unsafe { crate::tokenizer::build_command::get_vec_cstr_token(token) };
	if args.is_empty() {
		unsafe {
			crate::tokenizer::destroy_tokens::destroy_all_tokens(&mut (*shell));
		}
		std::process::exit(0);
	}
	let status = match token.cmd_name.as_slice() {
		b"echo" => echo(args),
		b"cd" => builtin_cd(&mut shell.env, args),
		b"pwd" => builtin_pwd(&shell.env),
		b"export" => builtin_export(&mut shell.env, args),
		b"unset" => builtin_unset(&mut shell.env, args),
		b"env" => builtin_env(&shell.env),
		b"exit" => builtin_exit(&mut shell.env, args),
		_ => exec_bin(&shell.env, &args),
	};
	shell.env.set_status(status);
}
