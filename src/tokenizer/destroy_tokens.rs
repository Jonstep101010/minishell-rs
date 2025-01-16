use ::libc;
use libutils_rs::src::utils::free_mem::free_null;

use crate::{t_arg, t_shell, t_token};

#[unsafe(no_mangle)]
pub unsafe fn destroy_all_tokens(mut shell: *mut t_shell) {
	let mut i = 0;
	while !(*shell).token.is_null() && i < (*shell).token_len {
		if !((*(*shell).token.add(i)).cmd_args).is_null() {
			let mut ii = 0;
			while !((*((*(*shell).token.add(i)).cmd_args).add(ii)).elem).is_null() {
				free_null(
					&mut (*((*(*shell).token.add(i)).cmd_args).add(ii)).elem
						as *mut *mut libc::c_char as *mut libc::c_void,
				);
				ii = ii.wrapping_add(1);
			}
			free_null(
				&mut (*(*shell).token.add(i)).cmd_args as *mut *mut t_arg as *mut libc::c_void,
			);
		}
		i = i.wrapping_add(1);
	}
	free_null(&mut (*shell).token as *mut *mut t_token as *mut libc::c_void);
	(*shell).token_len = 0;
}
