mod bin_path;
pub(crate) mod exec_bin;
mod execute_pipes;
mod heredoc;
mod redirections;
use self::{execute_pipes::execute_pipes, redirections::do_redirections};
#[allow(unused_imports)]
use crate::{
	builtins::{cd::builtin_cd, exit::builtin_exit, export::builtin_export, unset::builtin_unset},
	eprint_msh,
	libutils_rs::src::utils::memsize::memsize,
	t_shell, t_token,
	tokenizer::destroy_tokens::destroy_all_tokens,
};
use ::libc;
#[allow(unused_imports)]
use libc::strerror;

unsafe fn forkable_builtin(mut token: *mut t_token) -> bool {
	(*token).cmd_func != Some(builtin_exit as unsafe fn(&mut t_shell, *mut t_token) -> i32)
		&& (*token).cmd_func != Some(builtin_export as unsafe fn(&mut t_shell, *mut t_token) -> i32)
		&& (*token).cmd_func != Some(builtin_unset as unsafe fn(&mut t_shell, *mut t_token) -> i32)
		&& (*token).cmd_func != Some(builtin_cd as unsafe fn(&mut t_shell, *mut t_token) -> i32)
}
#[unsafe(no_mangle)]
pub unsafe fn execute_commands(mut shell: &mut t_shell) {
	let mut error_elem: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
	let mut token = shell.token;
	if token.is_null() {
		shell.exit_status = u8::MAX as i32;
		return;
	}
	shell.token_len = match memsize(
		shell.token as *mut libc::c_void,
		::core::mem::size_of::<t_token>() as libc::c_ulong,
	) as usize
	{
		0 => return,
		val => Some(val),
	};
	match shell.token_len.unwrap() {
		1 if !forkable_builtin(token) => {
			let mut redir_status = do_redirections((*token).cmd_args, &mut error_elem);
			if redir_status != 0 {
				if error_elem.is_null() {
					todo!("check the conditions!");
					// panic!("error_elem is null");
				} else {
					// @audit
					todo!("error printing!");
					// eprint_msh!("{}: {}", error_elem, err);
				};
				// (*shell).exit_status = redir_status as u8;
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
