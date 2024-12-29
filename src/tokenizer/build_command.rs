use super::token_utils::init_token;
use crate::{parser::split_outside_quotes::split_outside_quotes, t_token};
use ::libc;
use libc::free;
use libft_rs::ft_strdup::ft_strdup;
use libutils_rs::src::array::append_str::append_str_arr_free;
use libutils_rs::src::array::arr_free::arr_free;
use libutils_rs::src::array::arr_len::arr_len;

#[no_mangle]
pub unsafe extern "C" fn get_cmd_arr_token(mut token: *mut t_token) -> *mut *mut libc::c_char {
	let mut i: libc::c_int = 0;
	let mut cmd_arr: *mut *mut libc::c_char = std::ptr::null_mut::<*mut libc::c_char>();
	if token.is_null() || ((*token).cmd_args).is_null() {
		return std::ptr::null_mut::<*mut libc::c_char>();
	}
	if !((*((*token).cmd_args).offset(0 as libc::c_int as isize)).elem).is_null() {
		while !((*((*token).cmd_args).offset(i as isize)).elem).is_null() {
			if (*((*token).cmd_args).offset(i as isize)).type_0 as libc::c_uint
				!= crate::e_arg::REDIR as libc::c_int as libc::c_uint
			{
				cmd_arr = append_str_arr_free(
					cmd_arr,
					ft_strdup((*((*token).cmd_args).offset(i as isize)).elem),
				);
				if cmd_arr.is_null() {
					return std::ptr::null_mut::<*mut libc::c_char>();
				}
			}
			i += 1;
		}
	}
	cmd_arr
}
#[no_mangle]
pub unsafe extern "C" fn get_tokens(mut trimmed_line: *const libc::c_char) -> *mut t_token {
	let mut split_pipes: *mut *mut libc::c_char =
		split_outside_quotes(trimmed_line, b"|\0" as *const u8 as *const libc::c_char);
	if split_pipes.is_null() {
		crate::eprint_msh!("alloc fail!");
		return std::ptr::null_mut::<libc::c_void>() as *mut t_token;
	}
	if (*split_pipes).is_null() {
		arr_free(split_pipes);
		return std::ptr::null_mut::<libc::c_void>() as *mut t_token;
	}
	let mut token: *mut t_token = init_token(arr_len(split_pipes));
	if token.is_null() {
		crate::eprint_msh!("alloc fail token");
	}
	let mut i: libc::c_int = 0;
	while !token.is_null() && !(*split_pipes.offset(i as isize)).is_null() {
		let fresh0 = &mut (*token.offset(i as isize)).split_pipes;
		*fresh0 = *split_pipes.offset(i as isize);
		i += 1;
	}
	free(split_pipes as *mut libc::c_void);
	token
}
