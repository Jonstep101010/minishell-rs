use ::libc;
use libc::{free, getcwd, printf};

use crate::{environment::get_env::get_env, t_shell, t_token};

#[no_mangle]
pub unsafe extern "C" fn builtin_pwd(
	mut shell: *mut t_shell,
	mut _token_nullable: *mut t_token,
) -> libc::c_int {
	let mut tmp: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
	let mut env_pwd: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
	tmp = getcwd(std::ptr::null_mut::<libc::c_char>(), 0);
	env_pwd = get_env((*shell).env, b"PWD\0" as *const u8 as *const libc::c_char);
	if env_pwd.is_null() {
		printf(b"%s\n\0" as *const u8 as *const libc::c_char, tmp);
	} else {
		printf(b"%s\n\0" as *const u8 as *const libc::c_char, env_pwd);
	}
	if !tmp.is_null() {
		free(tmp as *mut libc::c_void);
	}
	if !env_pwd.is_null() {
		free(env_pwd as *mut libc::c_void);
	}
	0 as libc::c_int
}
