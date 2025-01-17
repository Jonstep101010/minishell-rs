use ::libc;
#[allow(unused_imports)]
use libc::{exit, free, strerror};
use libft_rs::ft_strchr::ft_strchr;
use libutils_rs::src::array::arr_free::arr_free;

use crate::{
	prelude::*, t_shell, t_token, tokenizer::build_command::get_cmd_arr_token,
	utils::exit_free::exit_free,
};

use super::bin_path::get_path_prefixed;
// unsafe fn execve_fail(mut shell: &mut t_shell, cmd: &CStr) {
// let err = CStr::from_ptr(where_it_happened);
// let err: &str = err.to_str().unwrap();
// eprint_msh!("{}{}", cmd.to_str().unwrap(), err);
// destroy_all_tokens(shell);
// free(shell as *mut libc::c_void);
// exit(where_it_happened);
// }
#[unsafe(no_mangle)]
pub unsafe fn exec_bin(mut shell: &mut t_shell, mut token: *mut t_token) -> i32 {
	// @note this might be a good candidate for implementing a rust version of the function
	let mut command: *mut *const libc::c_char =
		get_cmd_arr_token(token) as *mut *const libc::c_char;
	if command.is_null() {
		exit_free(shell, 0);
	}
	let env = &shell.env;
	if !(*command).is_null() {
		let (mut access_status, path_prefixed_bin) =
			get_path_prefixed(env, std::ffi::CStr::from_ptr(*command));
		if path_prefixed_bin.is_some() {
			(*token).bin = path_prefixed_bin.unwrap();
		}
		if access_status == 1 || access_status == 2 || access_status == 126 || access_status == 127
		{
			if access_status == 126
				&& (ft_strchr(
					b"~/\0" as *const u8 as *const libc::c_char,
					**command as i32,
				))
				.is_null()
			{
				// let cmd = i8const_str(command, 0);
				// @audit
				todo!("handle no perms");
				// eprint_msh!("{}: {}", cmd, err);
			}
			if access_status == 127 {
				eprint_msh!("{}: command not found", i8const_str(command, 0));
			}
			arr_free(command as *mut *mut libc::c_char);
			exit_free(shell, access_status.into());
		}
		if libc::execve(
			(*token).bin.as_ptr(),
			command as *mut *mut libc::c_char as *const *const libc::c_char,
			shell.env.as_ptr_array().as_ptr(),
		) == -1_i32
		{
			arr_free(command as *mut *mut libc::c_char);
			todo!("handle execve_fail");
			// execve_fail(shell, (*token).bin.as_c_str());
		}
		arr_free(command as *mut *mut libc::c_char);
		exit_free(shell, 0_i32);
		return 0_i32;
	}
	arr_free(command as *mut *mut libc::c_char);
	exit_free(shell, 1);
	1_i32
}
