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
	libutils_rs::src::utils::memsize::memsize,
	t_shell, t_token,
	tokenizer::destroy_tokens::destroy_all_tokens,
};
use ::libc;
use exec_bin::exec_bin;

#[unsafe(no_mangle)]
pub unsafe fn execute_commands(shell: &mut t_shell) {
	let token = shell.token;
	shell.token_len = match memsize(
		shell.token as *mut libc::c_void,
		::core::mem::size_of::<t_token>() as libc::c_ulong,
	) as usize
	{
		0 => return,
		val => Some(val),
	};
	match shell.token_len.unwrap() {
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
			// @note this might be a good candidate for implementing a rust version of the function
			let command: *mut *const libc::c_char =
				crate::tokenizer::build_command::get_cmd_arr_token(token)
					as *mut *const libc::c_char;
			if command.is_null() {
				crate::tokenizer::destroy_tokens::destroy_all_tokens(&mut (*shell));
				// free(shell as *mut libc::c_void);
				std::process::exit(0);
			}
			let status = executor(token, command, shell);
			shell.env.set_status(status);
		}
		_ => {
			execute_pipes(shell);
		}
	}
	destroy_all_tokens(&mut (*shell));
}

pub unsafe fn executor(token: *mut t_token, command: *mut *const i8, shell: &mut t_shell) -> i32 {
	let status = match (*token).cmd_name.as_slice() {
		b"echo" => echo(command),
		b"cd" => builtin_cd(&mut shell.env, command),
		b"pwd" => builtin_pwd(&shell.env),
		b"export" => builtin_export(&mut shell.env, command),
		b"unset" => builtin_unset(&mut shell.env, command),
		b"env" => builtin_env(&shell.env),
		b"exit" => builtin_exit(&mut shell.env, command),
		_ => exec_bin(&shell.env, command),
	};
	status
}
