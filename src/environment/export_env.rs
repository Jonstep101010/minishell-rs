use ::libc;

use crate::{
	builtins::exit::builtin_exit, environment::get_index::get_index_env, t_shell, t_token,
	utils::error::eprint,
};
use libc::free;
use libft_rs::ft_itoa::ft_itoa;
use libutils_rs::src::{
	array::append_str::append_str_arr_free, string::join_strings::free_second_join,
};
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
unsafe extern "C" fn update_var(mut env: *mut *mut libc::c_char, mut key_val: *mut libc::c_char) {
	if key_val.is_null() || env.is_null() || (*env).is_null() || *key_val == 0 {
		return;
	}
	free(*env as *mut libc::c_void);
	*env = key_val;
}
#[no_mangle]
pub unsafe extern "C" fn export_env(mut shell: *mut t_shell, mut key_val: *mut libc::c_char) {
	let mut index: libc::c_int = 0;
	if key_val.is_null() || *key_val == 0 {
		return eprint(
			b"export: malloc fail creating key_val\n\0" as *const u8 as *const libc::c_char,
		);
	}
	if shell.is_null()
		|| ((*shell).env).is_null()
		|| (*(*shell).env).is_null()
		|| **(*shell).env == 0
	{
		free(key_val as *mut libc::c_void);
		(*shell).exit_status = 1 as libc::c_int as uint8_t;
		eprint(b"fatal: invalid memory!\n\0" as *const u8 as *const libc::c_char);
		builtin_exit(shell, std::ptr::null_mut::<t_token>());
	}
	index = get_index_env((*shell).env, key_val);
	if index == -(1 as libc::c_int) {
		(*shell).env = append_str_arr_free((*shell).env, key_val);
	} else {
		update_var(&mut *((*shell).env).offset(index as isize), key_val);
	}
	if ((*shell).env).is_null() {
		(*shell).exit_status = 1 as libc::c_int as uint8_t;
		eprint(b"fatal: environment invalidated\n\0" as *const u8 as *const libc::c_char);
		builtin_exit(shell, std::ptr::null_mut::<t_token>());
	}
}
#[no_mangle]
pub unsafe extern "C" fn update_exit_status(mut shell: *mut t_shell, mut status: libc::c_int) {
	export_env(
		shell,
		free_second_join(b"?=\0" as *const u8 as *const libc::c_char, ft_itoa(status)),
	);
	(*shell).exit_status = status as uint8_t;
}
