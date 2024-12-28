mod bin_path;
pub(crate) mod exec_bin;
mod execute_pipes;
mod heredoc;
mod redirections;
use self::{execute_pipes::execute_pipes, redirections::do_redirections};
#[allow(unused_imports)]
use crate::{
	__errno_location,
	builtins::{cd::builtin_cd, exit::builtin_exit, export::builtin_export, unset::builtin_unset},
	eprint_msh,
	libutils_rs::src::utils::memsize::memsize,
	t_shell, t_token,
	tokenizer::destroy_tokens::destroy_all_tokens,
};
use ::libc;
#[allow(unused_imports)]
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
	let mut error_elem: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
	if token.is_null() {
		return update_exit_status(shell, -(1 as libc::c_int));
	}
	let mut token_count = memsize(
		(*shell).token as *mut libc::c_void,
		::core::mem::size_of::<t_token>() as libc::c_ulong,
	) as libc::c_int;
	if token_count == 1 as libc::c_int && !forkable_builtin(token) {
		let mut redir_status = do_redirections((*token).cmd_args, &mut error_elem);
		if redir_status != 0 as libc::c_int {
			if error_elem.is_null() {
				panic!("error_elem is null");
			} else {
				// @audit
				let error_elem = stringify!(error_elem);
				let err = stringify!(strerror(*__errno_location()));
				eprint_msh!("{}: {}", error_elem, err);
				// @audit
			};
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
