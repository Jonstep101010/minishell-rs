use ::libc;
#[allow(unused_imports)]
use libc::{exit, free, strerror};

#[allow(unused_imports)]
use crate::{
	__errno_location, libutils_rs::src::array::arr_free::arr_free, prelude::*, t_shell, t_token,
	tokenizer::destroy_tokens::destroy_all_tokens,
};

#[no_mangle]
pub unsafe extern "C" fn exit_free(mut shell: *mut t_shell, mut exit_code: libc::c_int) {
	destroy_all_tokens(shell);
	free(shell as *mut libc::c_void);
	exit(exit_code);
}
#[no_mangle]
pub unsafe extern "C" fn exit_error(mut shell: *mut t_shell, mut error_elem: *mut libc::c_char) {
	if !error_elem.is_null() {
		let error_elem = stringify!(error_elem);
		let error = stringify!(strerror(*__errno_location()));
		eprint_msh!("{}: {}", error_elem, error);
	}
	destroy_all_tokens(shell);
	free(shell as *mut libc::c_void);
	exit(*__errno_location());
}
