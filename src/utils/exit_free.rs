use ::libc;
use libc::{exit, free, strerror};

use crate::{
	__errno_location, libutils_rs::src::array::arr_free::arr_free, t_shell,
	tokenizer::destroy_tokens::destroy_all_tokens, utils::error::eprint,
};

#[no_mangle]
pub unsafe extern "C" fn exit_free(mut shell: *mut t_shell, mut exit_code: libc::c_int) {
	if !((*shell).env).is_null() {
		arr_free((*shell).env);
	}
	destroy_all_tokens(shell);
	free(shell as *mut libc::c_void);
	exit(exit_code);
}
#[no_mangle]
pub unsafe extern "C" fn exit_error(mut shell: *mut t_shell, mut error_elem: *mut libc::c_char) {
	let mut error: *mut libc::c_char = strerror(*__errno_location());
	if !error_elem.is_null() {
		eprint(
			b"%s: %s\0" as *const u8 as *const libc::c_char,
			error_elem,
			error,
		);
	}
	if !((*shell).env).is_null() {
		arr_free((*shell).env);
	}
	destroy_all_tokens(shell);
	free(shell as *mut libc::c_void);
	exit(*__errno_location());
}
