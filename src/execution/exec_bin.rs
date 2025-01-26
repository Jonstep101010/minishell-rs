use ::libc;
use libft_rs::ft_strchr::ft_strchr;
use libutils_rs::src::array::arr_free::arr_free;

use crate::prelude::*;

use super::bin_path::get_path_prefixed;

#[unsafe(no_mangle)]
pub unsafe fn exec_bin(shell_env: &Env, command: *mut *const c_char) -> i32 {
	let mut path_exec_bin = None;
	let (access_status, path_prefixed_bin) =
		get_path_prefixed(shell_env, std::ffi::CStr::from_ptr(*command));
	if path_prefixed_bin.is_some() {
		path_exec_bin = path_prefixed_bin;
	}
	if access_status == 1 || access_status == 2 || access_status == 126 || access_status == 127 {
		if access_status == 126 && (ft_strchr(c"~/".as_ptr(), **command as i32)).is_null() {
			// let cmd = i8const_str(command, 0);
			// @audit
			todo!("handle no perms");
			// eprint_msh!("{}: {}", cmd, err);
		}
		if access_status == 127 {
			eprint_msh!("{}: command not found", i8const_str(command, 0));
		}
		arr_free(command as *mut *mut libc::c_char);
		std::process::exit(access_status.into());
	}
	if libc::execve(
		path_exec_bin.unwrap().as_ptr(),
		command as *mut *mut libc::c_char as *const *const libc::c_char,
		shell_env.as_ptr_array().as_ptr(),
	) == -1
	{
		todo!("handle execve_fail");
	}
	arr_free(command as *mut *mut libc::c_char);
	std::process::exit(0);
}
