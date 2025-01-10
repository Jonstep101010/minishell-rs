use crate::{eprint_msh, size_t};
use ::libc;
use libft_rs::{ft_isalnum::ft_isalnum, ft_strchr::ft_strchr};
use libutils_rs::src::char::is_something::ft_isspace;

use super::t_lexer;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct s_check_pipes {
	pub flag_redir: libc::c_int,
	pub flag_word: libc::c_int,
	pub i: size_t,
	pub ignore: bool,
}

unsafe extern "C" fn inner_while_quotes(
	mut s: *const libc::c_char,
	mut input: *mut t_lexer,
	mut check: *mut s_check_pipes,
) -> libc::c_int {
	(*check).flag_word = 0 as libc::c_int;
	(*check).flag_redir = 0 as libc::c_int;
	while *s.offset((*check).i as isize) as libc::c_int != 0
		&& *s.offset((*check).i as isize) as libc::c_int != '|' as i32
		&& !*((*input).ignore).offset((*check).i as isize)
	{
		if !(ft_strchr(
			b"><\0" as *const u8 as *const libc::c_char,
			*s.offset((*check).i as isize) as libc::c_int,
		))
		.is_null() && ((*check).flag_redir == 0
			|| *s.offset(((*check).i).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
				as libc::c_int
				!= 0 && *s
				.offset(((*check).i).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
				as libc::c_int
				== *s.offset((*check).i as isize) as libc::c_int)
		{
			(*check).flag_redir = 1 as libc::c_int;
		} else if !(ft_strchr(
			b"><\0" as *const u8 as *const libc::c_char,
			*s.offset((*check).i as isize) as libc::c_int,
		))
		.is_null()
		{
			eprint_msh!("syntax error near unexpected token `newline'");
			return 2 as libc::c_int;
		} else if ft_isalnum(*s.offset((*check).i as isize) as libc::c_int) != 0 {
			(*check).flag_redir = 0 as libc::c_int;
			(*check).flag_word = 1 as libc::c_int;
		}
		(*check).i = ((*check).i).wrapping_add(1);
	}
	0 as libc::c_int
}
unsafe extern "C" fn inner_if_quotes(
	mut s: *const libc::c_char,
	mut input: *mut t_lexer,
	mut check: *mut s_check_pipes,
) -> libc::c_int {
	if !*((*input).ignore).offset((*check).i as isize)
		&& *s.offset((*check).i as isize) as libc::c_int == '|' as i32
		&& !(*check).ignore
	{
		if (*check).flag_word == 0 {
			eprint_msh!("syntax error near unexpected token `|'");
			return 2 as libc::c_int;
		}
		if (*s.offset((*check).i as isize) == 0
			|| *s.offset((*check).i as isize) as libc::c_int == '|' as i32)
			&& ((*check).flag_redir != 0 || (*check).flag_word == 0)
		{
			eprint_msh!("syntax error near unexpected token `|'");
			return 2 as libc::c_int;
		}
	}
	if *s.offset((*check).i as isize) as libc::c_int == '|' as i32 {
		(*check).ignore = 0 as libc::c_int != 0;
	}
	0 as libc::c_int
}
unsafe extern "C" fn check_pipes_redirection_quotes(
	mut s: *const libc::c_char,
	mut input: *mut t_lexer,
	mut check: *mut s_check_pipes,
) -> libc::c_int {
	while (*check).i < (*input).len && !((*input).ignore).is_null() {
		if !*((*input).ignore).offset((*check).i as isize)
			&& (inner_while_quotes(s, input, check) != 0 as libc::c_int
				|| inner_if_quotes(s, input, check) != 0 as libc::c_int)
		{
			return 2 as libc::c_int;
		}
		if *((*input).ignore).offset((*check).i as isize) {
			(*check).ignore = 1 as libc::c_int != 0;
			while *s.offset((*check).i as isize) as libc::c_int != 0
				&& *((*input).ignore).offset((*check).i as isize) as libc::c_int != 0
			{
				(*check).i = ((*check).i).wrapping_add(1);
			}
		} else {
			(*check).i = ((*check).i).wrapping_add(1);
		}
	}
	if (*check).flag_redir != 0 && !(*check).ignore {
		eprint_msh!("syntax error near unexpected token `newline'");
		return 2 as libc::c_int;
	}
	0 as libc::c_int
}
unsafe extern "C" fn inner_while_noquotes(
	mut s: *const libc::c_char,
	mut check: *mut s_check_pipes,
) -> libc::c_int {
	while *s.offset((*check).i as isize) as libc::c_int != 0
		&& *s.offset((*check).i as isize) as libc::c_int != '|' as i32
	{
		if !(ft_strchr(
			b"><\0" as *const u8 as *const libc::c_char,
			*s.offset((*check).i as isize) as libc::c_int,
		))
		.is_null() && ((*check).flag_redir == 0
			|| *s.offset(((*check).i).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
				as libc::c_int
				!= 0 && *s
				.offset(((*check).i).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
				as libc::c_int
				== *s.offset((*check).i as isize) as libc::c_int
				&& (*s
					.offset(((*check).i).wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize)
					== 0 || ft_isspace(
					*s.offset(((*check).i).wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize)
						as libc::c_int,
				) != 0))
		{
			(*check).flag_redir = 1 as libc::c_int;
		} else if !(ft_strchr(
			b"><\0" as *const u8 as *const libc::c_char,
			*s.offset((*check).i as isize) as libc::c_int,
		))
		.is_null()
		{
			eprint_msh!("syntax error near unexpected token `newline'");
			return 2 as libc::c_int;
		} else if ft_isalnum(*s.offset((*check).i as isize) as libc::c_int) != 0 {
			(*check).flag_redir = 0 as libc::c_int;
			(*check).flag_word = 1 as libc::c_int;
		}
		(*check).i = ((*check).i).wrapping_add(1);
	}
	0 as libc::c_int
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn check_pipes_redirection(
	mut s: *const libc::c_char,
	mut input: *mut t_lexer,
) -> libc::c_int {
	let mut check: s_check_pipes = s_check_pipes {
		flag_redir: 0,
		flag_word: 0,
		i: 0,
		ignore: false,
	};
	if *s as libc::c_int == '|' as i32
		|| *s.offset(((*input).len).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
			as libc::c_int
			== '|' as i32
	{
		eprint_msh!("syntax error near unexpected token `|'");
		return 2 as libc::c_int;
	}
	if !(ft_strchr(
		b"<>\0" as *const u8 as *const libc::c_char,
		*s.offset(((*input).len).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
			as libc::c_int,
	))
	.is_null()
	{
		eprint_msh!("syntax error near unexpected token `newline'");
		return 2 as libc::c_int;
	}
	if !((*input).ignore).is_null() {
		return check_pipes_redirection_quotes(s, input, &mut check);
	}
	while check.i < (*input).len {
		check = {
			s_check_pipes {
				flag_redir: 0,
				flag_word: 0,
				i: check.i,
				ignore: false,
			}
		};
		if inner_while_noquotes(s, &mut check) != 0 as libc::c_int {
			return 2 as libc::c_int;
		}
		if check.flag_word == 0 {
			eprint_msh!("syntax error near unexpected token `|'");
			return 2 as libc::c_int;
		}
		if (*s.offset(check.i as isize) == 0
			|| *s.offset(check.i as isize) as libc::c_int == '|' as i32)
			&& (check.flag_redir != 0 || check.flag_word == 0)
		{
			eprint_msh!("syntax error near unexpected token `|'");
			return 2 as libc::c_int;
		}
		while *s.offset(check.i as isize) as libc::c_int != 0
			&& *s.offset(check.i as isize) as libc::c_int == '|' as i32
		{
			check.i = (check.i).wrapping_add(1);
		}
	}
	if check.flag_redir != 0 {
		crate::eprint_msh!("syntax error near unexpected token `newline'");
		return 2 as libc::c_int;
	}
	0 as libc::c_int
}
