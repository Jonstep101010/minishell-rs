use ::libc;
use libutils_rs::src::utils::free_mem::free_null;

use crate::{t_shell, t_token};

#[unsafe(no_mangle)]
pub unsafe fn destroy_all_tokens(mut shell: &mut t_shell) {
	let mut i = 0;
	if shell.token_len.is_some() {
		while i < shell.token_len.unwrap() {
			if !((*shell.token.add(i)).cmd_args_vec).is_empty() {
				let mut ii = 0;
				while ii < (*shell.token.add(i)).cmd_args_vec.len() {
					free_null(
						&mut (*shell.token.add(i)).cmd_args_vec[ii].elem as *mut *mut libc::c_char
							as *mut libc::c_void,
					);
					ii += 1;
				}
			}
			i += 1;
		}
	}
	free_null(&mut shell.token as *mut *mut t_token as *mut libc::c_void);
	shell.token_len = None;
}
