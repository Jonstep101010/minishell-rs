use ::libc;
use libutils_rs::src::utils::free_mem::free_null;

use crate::{size_t, t_arg, t_shell, t_token};

#[no_mangle]
pub unsafe extern "C" fn destroy_all_tokens(mut shell: *mut t_shell) {
	let mut token: *mut t_token = std::ptr::null_mut::<t_token>();
	let mut i: size_t = 0;
	let mut ii: size_t = 0;
	token = (*shell).token;
	i = 0 as libc::c_int as size_t;
	while !token.is_null() && i < (*shell).token_len {
		if !((*token.offset(i as isize)).cmd_args).is_null() {
			ii = 0 as libc::c_int as size_t;
			while !((*((*token.offset(i as isize)).cmd_args).offset(ii as isize)).elem).is_null() {
				free_null(
					&mut (*((*token.offset(i as isize)).cmd_args).offset(ii as isize)).elem
						as *mut *mut libc::c_char as *mut libc::c_void,
				);
				ii = ii.wrapping_add(1);
			}
			free_null(
				&mut (*token.offset(i as isize)).cmd_args as *mut *mut t_arg as *mut libc::c_void,
			);
		}
		if !((*token.offset(i as isize)).bin).is_null() {
			free_null(
				&mut (*token.offset(i as isize)).bin as *mut *mut libc::c_char as *mut libc::c_void,
			);
		}
		i = i.wrapping_add(1);
	}
	free_null(&mut (*shell).token as *mut *mut t_token as *mut libc::c_void);
	(*shell).token_len = 0 as libc::c_int as size_t;
}
