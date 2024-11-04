mod bin_path;
pub(crate) mod exec_bin;
mod execute_pipes;
mod heredoc;
mod redirections;
use self::{execute_pipes::execute_pipes, redirections::do_redirections};
use crate::{
	__errno_location,
	builtins::{cd::builtin_cd, exit::builtin_exit, export::builtin_export, unset::builtin_unset},
	environment::export_env::update_exit_status,
	libutils_rs::src::utils::memsize::memsize,
	t_shell, t_token,
	tokenizer::destroy_tokens::destroy_all_tokens,
	utils::error::eprint,
};
use ::libc;
use libc::strerror;

unsafe extern "C" fn forkable_builtin(mut token: *mut t_token) -> bool {
	(*token).cmd_func
		!= Some(builtin_exit as unsafe extern "C" fn(*mut t_shell, *mut t_token) -> libc::c_int)
		&& (*token).cmd_func
			!= Some(
				builtin_export as unsafe extern "C" fn(*mut t_shell, *mut t_token) -> libc::c_int,
			) && (*token).cmd_func
		!= Some(builtin_unset as unsafe extern "C" fn(*mut t_shell, *mut t_token) -> libc::c_int)
		&& (*token).cmd_func
			!= Some(builtin_cd as unsafe extern "C" fn(*mut t_shell, *mut t_token) -> libc::c_int)
}
#[no_mangle]
pub unsafe extern "C" fn execute_commands(mut shell: *mut t_shell, mut token: *mut t_token) {
	let mut token_count: libc::c_int = 0;
	let mut redir_status: libc::c_int = 0;
	let mut error_elem: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
	if token.is_null() {
		return update_exit_status(shell, -(1 as libc::c_int));
	}
	token_count = memsize(
		(*shell).token as *mut libc::c_void,
		::core::mem::size_of::<t_token>() as libc::c_ulong,
	) as libc::c_int;
	if token_count == 1 as libc::c_int && !forkable_builtin(token) {
		redir_status = do_redirections((*token).cmd_args, &mut error_elem);
		if redir_status != 0 as libc::c_int {
			eprint(
				b"%s: %s\0" as *const u8 as *const libc::c_char,
				error_elem,
				strerror(*__errno_location()),
			);
			return update_exit_status(shell, redir_status);
		}
		update_exit_status(
			shell,
			((*token).cmd_func).expect("non-null function pointer")(shell, token),
		);
	} else {
		execute_pipes(shell, token_count);
	}
	destroy_all_tokens(shell);
}
