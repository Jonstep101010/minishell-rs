use ::libc;

use crate::{t_shell, t_token};

#[unsafe(no_mangle)]
pub unsafe fn destroy_all_tokens(shell: &mut t_shell) {
	let mut i = 0;
	if shell.token_len.is_some() {
		while i < shell.token_len.unwrap() {
			if !((*shell.token.add(i)).cmd_args_vec).is_empty() {
				// let mut ii = 0;
				// while ii < (*shell.token.add(i)).cmd_args_vec.len() {
				// 	free_null(
				// 		&mut (*shell.token.add(i)).cmd_args_vec[ii].elem as *mut *mut libc::c_char
				// 			as *mut libc::c_void,
				// 	);
				// 	ii += 1;
				// }
			}
			i += 1;
		}
	}
	{
		let p = &mut shell.token as *mut *mut t_token as *mut libc::c_void;
		if p.is_null() {
			return;
		}
		let ptr: *mut *mut libc::c_void = p as *mut *mut libc::c_void;
		if !(*ptr).is_null() {
			libc::free(*ptr);
		}
		*ptr = std::ptr::null_mut::<libc::c_void>();
	};
	shell.token_len = None;
}
