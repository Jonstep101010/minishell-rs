use ::libc;
#[allow(unused_imports)]
use libc::{exit, free, strerror};

#[allow(unused_imports)]
use crate::{
	libutils_rs::src::array::arr_free::arr_free, prelude::*, t_shell, t_token,
	tokenizer::destroy_tokens::destroy_all_tokens,
};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn exit_free(mut shell: *mut t_shell, mut exit_code: libc::c_int) {
	destroy_all_tokens(shell);
	free(shell as *mut libc::c_void);
	exit(exit_code);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn exit_error(mut shell: *mut t_shell, mut error_elem: *mut libc::c_char) {
	if !error_elem.is_null() {
		todo!("display error");
		// eprint_msh!("{}: {}", error_elem, error);
	}
	destroy_all_tokens(shell);
	free(shell as *mut libc::c_void);
	todo!("bail out gracefully");
}
