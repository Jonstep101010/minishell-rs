use std::ffi::CString;

use libft_rs::ft_strchr::ft_strchr;

use crate::prelude::*;

use super::bin_path::get_path_prefixed;

#[unsafe(no_mangle)]
pub unsafe fn exec_bin(shell_env: &Env, slice_args: &[CString]) -> i32 {
	let mut path_exec_bin = None;
	let (access_status, path_prefixed_bin) = get_path_prefixed(shell_env, &slice_args[0]);
	if path_prefixed_bin.is_some() {
		path_exec_bin = path_prefixed_bin;
	}
	if access_status == 1 || access_status == 2 || access_status == 126 || access_status == 127 {
		if access_status == 126
			&& (ft_strchr(c"~/".as_ptr(), slice_args[0].as_bytes()[0] as i32)).is_null()
		{
			// let cmd = i8const_str(command, 0);
			// @audit
			todo!("handle no perms");
			// eprint_msh!("{}: {}", cmd, err);
		}
		if access_status == 127 {
			eprint_msh!("{:?}: command not found", slice_args);
		}
		// arr_free(command as *mut *mut libc::c_char);
		std::process::exit(access_status.into());
	}
	// if libc::execve(
	// 	path_exec_bin.unwrap().as_ptr(),
	// 	command as *mut *mut libc::c_char as *const *const libc::c_char,
	// 	shell_env.as_ptr_array().as_ptr(),
	// ) == -1
	// {
	// 	todo!("handle execve_fail");
	// }
	let slice_env = shell_env.to_cstring_vec();
	if nix::unistd::execve(&path_exec_bin.unwrap(), slice_args, slice_env.as_slice()).is_err() {
		todo!("handle execve_fail");
	}
	// arr_free(command as *mut *mut libc::c_char);
	std::process::exit(0);
}
