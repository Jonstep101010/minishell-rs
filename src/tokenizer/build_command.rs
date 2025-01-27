use crate::prelude::*;
use ::libc;
use libft_rs::ft_strdup::ft_strdup;
use libutils_rs::src::array::append_str::append_str_arr_free;

#[unsafe(no_mangle)]
///
/// `returns` execve style argv
pub unsafe fn get_cmd_arr_token(token: *mut t_token) -> *mut *mut libc::c_char {
	let mut i = 0;
	let mut cmd_arr: *mut *mut libc::c_char = std::ptr::null_mut::<*mut libc::c_char>();
	if token.is_null()
		|| ((*token).cmd_args_vec).is_empty()
		|| ((*token).cmd_args_vec[0]).elem.is_null()
	{
		panic!("token for get_cmd_arr shall not be null")
	}
	while i < (*token).cmd_args_vec.len() {
		if (*token).cmd_args_vec[i].type_0 != REDIR {
			cmd_arr = append_str_arr_free(cmd_arr, ft_strdup((*token).cmd_args_vec[i].elem));
			if cmd_arr.is_null() {
				return std::ptr::null_mut::<*mut libc::c_char>();
			}
		}
		i += 1;
	}
	cmd_arr
}

use std::ffi::{CStr, CString};
pub unsafe fn get_vec_cstr_token(token: *mut t_token) -> Vec<CString> {
	let mut i = 0;
	if token.is_null()
		|| ((*token).cmd_args_vec).is_empty()
		|| ((*token).cmd_args_vec[0]).elem.is_null()
	{
		panic!("token for get_cmd_arr shall not be null")
	}
	let mut vec_cstr = Vec::new();
	while i < (*token).cmd_args_vec.len() {
		if (*token).cmd_args_vec[i].type_0 != REDIR {
			let cstr_ref = CStr::from_ptr((*token).cmd_args_vec[i].elem);
			let cstring = CString::from(cstr_ref);
			vec_cstr.push(cstring);
		}
		i += 1;
	}
	vec_cstr
}
