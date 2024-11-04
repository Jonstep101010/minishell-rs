use ::libc;
use libc::free;
use libft_rs::{ft_calloc::ft_calloc, ft_strlen::ft_strlen};

use crate::utils::{
	bool_array::{bool_arr_zeroing, range_ignore},
	error::eprint_single,
};

use super::{check_pipes::check_pipes_redirection, lexer_support::count_number, t_lexer};

unsafe extern "C" fn ignore_quotes(
	mut s: *const libc::c_char,
	mut input: *mut t_lexer,
) -> libc::c_int {
	if s.is_null() || input.is_null() {
		return 1 as libc::c_int;
	}
	(*input).ignore = bool_arr_zeroing(ft_strlen(s));
	range_ignore(s, (*input).ignore, '"' as i32 as libc::c_uchar);
	range_ignore(s, (*input).ignore, '\'' as i32 as libc::c_uchar);
	0 as libc::c_int
}
unsafe extern "C" fn check_quotes(
	mut s: *const libc::c_char,
	mut input: *mut t_lexer,
) -> libc::c_int {
	if (*input).singlequotes == 1 as libc::c_int {
		eprint_single(
			b"syntax error near unexpected token '''\n\0" as *const u8 as *const libc::c_char,
		);
		return 1 as libc::c_int;
	}
	if (*input).doublequotes == 1 as libc::c_int {
		eprint_single(
			b"syntax error near unexpected token '\"'\n\0" as *const u8 as *const libc::c_char,
		);
		return 1 as libc::c_int;
	}
	if (*input).singlequotes % 2 as libc::c_int != 0 as libc::c_int
		|| (*input).doublequotes % 2 as libc::c_int != 0 as libc::c_int
	{
		eprint_single(b"error: quotes not closed\0" as *const u8 as *const libc::c_char);
		return 1 as libc::c_int;
	}
	if (*input).singlequotes > 0 as libc::c_int || (*input).doublequotes > 0 as libc::c_int {
		ignore_quotes(s, input);
	}
	0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn lexer_checks_basic(mut s: *const libc::c_char) -> *mut t_lexer {
	let mut input: *mut t_lexer = std::ptr::null_mut::<t_lexer>();
	input = ft_calloc(
		::core::mem::size_of::<t_lexer>() as libc::c_ulong,
		1 as libc::c_int as crate::size_t,
	) as *mut t_lexer;
	count_number(s, input);
	(*input).lexer = check_quotes(s, input);
	if (*input).lexer != 0 as libc::c_int {
		(*input).lexer = 0 as libc::c_int;
		free((*input).ignore as *mut libc::c_void);
		return input;
	}
	if (*input).pipes != 0 || (*input).redir_greater != 0 || (*input).redir_smaller != 0 {
		(*input).lexer = check_pipes_redirection(s, input);
		if (*input).lexer != 0 as libc::c_int {
			free((*input).ignore as *mut libc::c_void);
			return input;
		}
	}
	(*input).result = 1 as libc::c_int != 0;
	free((*input).ignore as *mut libc::c_void);
	input
}
