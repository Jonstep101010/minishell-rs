use crate::prelude::*;
use ::libc;
use libft_rs::ft_strdup::ft_strdup;
use libutils_rs::src::array::append_str::append_str_arr_free;

#[unsafe(no_mangle)]
///
/// `returns` execve style argv
pub unsafe fn get_cmd_arr_token(mut token: *mut t_token) -> *mut *mut libc::c_char {
	let mut i = 0;
	let mut cmd_arr: *mut *mut libc::c_char = std::ptr::null_mut::<*mut libc::c_char>();
	if token.is_null() || ((*token).cmd_args).is_null() || ((*(*token).cmd_args).elem).is_null() {
		panic!("token for get_cmd_arr shall not be null")
	}
	while !((*((*token).cmd_args).add(i)).elem).is_null() {
		if (*((*token).cmd_args).add(i)).type_0 != REDIR {
			cmd_arr = append_str_arr_free(cmd_arr, ft_strdup((*((*token).cmd_args).add(i)).elem));
			if cmd_arr.is_null() {
				return std::ptr::null_mut::<*mut libc::c_char>();
			}
		}
		i += 1;
	}
	cmd_arr
}
