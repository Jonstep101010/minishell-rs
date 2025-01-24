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
#[allow(unused_imports)]
use libc::strerror;

#[unsafe(no_mangle)]
pub unsafe fn execute_commands(mut shell: &mut t_shell) {
	let mut token = shell.token;
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
			if do_redirections((*token).cmd_args).is_err() {
				todo!("some sort of handling");
			}
			shell.exit_status =
				((*token).cmd_func).expect("non-null function pointer")(shell, token) as u8 as i32;
		}
		_ => {
			execute_pipes(shell);
		}
	}
	destroy_all_tokens(&mut (*shell));
}
