#[allow(unused_imports)]
use std::ffi::{CStr, CString};

use ::libc;
use libc::free;
use libft_rs::{ft_strlen::ft_strlen, ft_strncmp::ft_strncmp};
use libutils_rs::src::{
	array::{arr_free::arr_free, arr_len::arr_len},
	string::str_cchr::str_cchr,
	utils::free_mem::free_null,
};

use crate::{
	builtins::env::builtin_env,
	environment::{Env, expander::expander},
	parser::{interpret_quotes::do_quote_bs, split_outside_quotes::split_outside_quotes},
	prelude::*,
	size_t, t_arg, t_shell, t_token,
};

use super::{
	build_command::get_tokens,
	destroy_tokens::destroy_all_tokens,
	redirection_utils::{check_redirections, parse_redir_types, rm_prefix_redir_word},
	token_utils::{init_cmdargs, set_cmd_func},
};

unsafe extern "C" fn expand_if_allowed(
	mut token: *mut t_token,
	mut ii: size_t,
	env: &Env,
) -> *mut libc::c_void {
	if (*token).cmd_func
		!= Some(builtin_env as unsafe extern "C" fn(*mut t_shell, *mut t_token) -> libc::c_int)
		&& str_cchr(
			(*((*token).cmd_args).offset(ii as isize)).elem,
			'$' as i32 as libc::c_char,
		) != 0 as libc::c_int
	{
		// we know this is non-null
		let c_str = CStr::from_ptr((*((*token).cmd_args).offset(ii as isize)).elem);
		let mut tmp = expander(c_str, env);
		if tmp.is_none() {
			return std::ptr::null_mut::<libc::c_void>();
		}
		let tmp = tmp.unwrap();
		if ft_strncmp(
			tmp.as_ptr(),
			(*((*token).cmd_args).offset(ii as isize)).elem,
			((if ft_strlen(tmp.as_ptr())
				> ft_strlen((*((*token).cmd_args).offset(ii as isize)).elem)
			{
				ft_strlen(tmp.as_ptr())
			} else {
				ft_strlen((*((*token).cmd_args).offset(ii as isize)).elem)
			}) == 0 as libc::c_int as libc::c_ulong) as libc::c_int as size_t,
		) != 0
		{
			// we need to make sure we do not free using free @audit
			// free(tmp as *mut libc::c_void);
		} else {
			free((*((*token).cmd_args).offset(ii as isize)).elem as *mut libc::c_void);
			let fresh0 = &mut (*((*token).cmd_args).offset(ii as isize)).elem;
			*fresh0 = tmp.into_raw();
		}
	}
	token as *mut libc::c_void
}
unsafe extern "C" fn setup_token(mut token: *mut t_token, env: &Env) -> *mut libc::c_void {
	if token.is_null() || ((*token).split_pipes).is_null() {
		return std::ptr::null_mut::<libc::c_void>();
	}
	(*token).tmp_arr = split_outside_quotes((*token).split_pipes, c" \t\n\r\x0B\x0C".as_ptr());
	free_null(&mut (*token).split_pipes as *mut *mut libc::c_char as *mut libc::c_void);
	if ((*token).tmp_arr).is_null() {
		return std::ptr::null_mut::<libc::c_void>();
	}
	(*token).cmd_args = init_cmdargs(arr_len((*token).tmp_arr));
	if ((*token).cmd_args).is_null() {
		arr_free((*token).tmp_arr);
		return std::ptr::null_mut::<libc::c_void>();
	}
	let mut ii: size_t = 0;
	while !(*((*token).tmp_arr).offset(ii as isize)).is_null() {
		let fresh1 = &mut (*((*token).cmd_args).offset(ii as isize)).elem;
		*fresh1 = *((*token).tmp_arr).offset(ii as isize);
		if (expand_if_allowed(token, ii, env)).is_null() {
			return std::ptr::null_mut::<libc::c_void>();
		}
		ii = ii.wrapping_add(1);
	}
	free_null(&mut (*token).tmp_arr as *mut *mut *mut libc::c_char as *mut libc::c_void);
	token as *mut libc::c_void
}
unsafe extern "C" fn rm_quotes(mut cmd_arg: *mut t_arg) {
	let mut quote: libc::c_int = 0;
	let mut i: libc::c_int = -1;
	loop {
		i += 1;
		if ((*cmd_arg.offset(i as isize)).elem).is_null() {
			break;
		}
		let mut tmp: *mut libc::c_char =
			do_quote_bs((*cmd_arg.offset(i as isize)).elem, &mut quote) as *mut libc::c_char;
		if tmp.is_null() {
			return;
		}
		free_null(
			&mut (*cmd_arg.offset(i as isize)).elem as *mut *mut libc::c_char as *mut libc::c_void,
		);
		let fresh2 = &mut (*cmd_arg.offset(i as isize)).elem;
		*fresh2 = tmp;
	}
}
unsafe extern "C" fn inner_loop(mut token: *mut t_token) -> *mut libc::c_void {
	if check_redirections((*token).cmd_args) {
		(*token).has_redir = 1 as libc::c_int != 0;
		parse_redir_types((*token).cmd_args);
		rm_prefix_redir_word((*token).cmd_args);
	}
	let mut i: libc::c_int = 0;
	while !((*((*token).cmd_args).offset(i as isize)).elem).is_null() {
		if (*((*token).cmd_args).offset(i as isize)).type_0 as libc::c_uint
			!= e_arg::REDIR as libc::c_int as libc::c_uint
		{
			break;
		}
		i += 1;
	}
	set_cmd_func((*((*token).cmd_args).offset(i as isize)).elem, token);
	rm_quotes((*token).cmd_args);
	token as *mut libc::c_void
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tokenize(
	mut shell: *mut t_shell,
	mut trimmed_line: *const libc::c_char,
) -> *mut libc::c_void {
	let mut i: size_t = 0;
	(*shell).token_len = 0 as libc::c_int as size_t;
	(*shell).token = get_tokens(trimmed_line);
	if ((*shell).token).is_null() {
		return std::ptr::null_mut::<libc::c_void>();
	}
	while !((*((*shell).token).offset((*shell).token_len as isize)).split_pipes).is_null() {
		(*shell).token_len = ((*shell).token_len).wrapping_add(1);
	}
	let shell_env = &(*shell).env;
	while i < (*shell).token_len {
		if setup_token(&mut *((*shell).token).offset(i as isize), shell_env).is_null() {
			destroy_all_tokens(shell);
			return std::ptr::null_mut::<libc::c_void>();
		}
		inner_loop(&mut *((*shell).token).offset(i as isize));
		i = i.wrapping_add(1);
	}
	(*shell).token as *mut libc::c_void
}
