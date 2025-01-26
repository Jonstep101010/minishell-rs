mod bin_path;
pub(crate) mod exec_bin;
mod execute_pipes;
mod heredoc;
mod redirections;
use self::{execute_pipes::execute_pipes, redirections::do_redirections};
use crate::{
	builtins::{echo::echo, env::builtin_env, pwd::builtin_pwd},
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
			(*token).cmd_func == Some(echo)
				|| (*token).cmd_func == Some(builtin_pwd)
				|| (*token).cmd_func == Some(builtin_env)
				|| (*token).cmd_func == Some(exec_bin)
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
			let status = ((*token).cmd_func).expect("non-null function pointer")(
				&mut (shell.env),
				Some(command),
			) as u8 as i32;
			shell.env.set_status(status);
		}
		_ => {
			execute_pipes(shell);
		}
	}
	destroy_all_tokens(&mut (*shell));
}
