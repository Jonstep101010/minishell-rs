use ::libc;

use crate::{t_shell, t_token};
use libc::printf;

#[no_mangle]
pub unsafe extern "C" fn builtin_env(
	mut shell: *mut t_shell,
	mut _token: *mut t_token,
) -> libc::c_int {
	let mut env: *const *const libc::c_char = (*shell).env as *const *const libc::c_char;
	while !(*env).is_null() {
		printf(b"%s\n\0" as *const u8 as *const libc::c_char, *env);
		env = env.offset(1);
	}
	0 as libc::c_int
}
