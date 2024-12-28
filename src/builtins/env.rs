use ::libc;

use crate::{t_shell, t_token};

#[no_mangle]
pub unsafe extern "C" fn builtin_env(
	mut shell: *mut t_shell,
	mut _token: *mut t_token,
) -> libc::c_int {
	print!("{}", (*shell).env);
	0 as libc::c_int
}
