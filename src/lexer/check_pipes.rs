use ::libc;
extern "C" {
	fn ft_isalnum(c: libc::c_int) -> libc::c_int;
	fn ft_strchr(str: *const libc::c_char, c: libc::c_int) -> *mut libc::c_char;
	fn ft_isspace(c: libc::c_int) -> libc::c_int;
	fn eprint(fmt: *const libc::c_char, _: ...);
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct s_check_pipes {
	pub flag_redir: libc::c_int,
	pub flag_word: libc::c_int,
	pub i: size_t,
	pub ignore: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct s_lexer {
	pub singlequotes: libc::c_int,
	pub doublequotes: libc::c_int,
	pub open_curly_brackets: libc::c_int,
	pub close_curly_brackets: libc::c_int,
	pub open_square_brackets: libc::c_int,
	pub close_square_brackets: libc::c_int,
	pub open_parentheses: libc::c_int,
	pub close_parentheses: libc::c_int,
	pub redir_greater: libc::c_int,
	pub redir_smaller: libc::c_int,
	pub pipes: libc::c_int,
	pub ignore: *mut bool,
	pub len: size_t,
	pub lexer: libc::c_int,
	pub result: bool,
}
pub type t_lexer = s_lexer;
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
				as libc::c_int != 0
				&& *s.offset(((*check).i).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
					as libc::c_int == *s.offset((*check).i as isize) as libc::c_int)
		{
			(*check).flag_redir = 1 as libc::c_int;
		} else if !(ft_strchr(
			b"><\0" as *const u8 as *const libc::c_char,
			*s.offset((*check).i as isize) as libc::c_int,
		))
		.is_null()
		{
			eprint(
				b"%s %s\0" as *const u8 as *const libc::c_char,
				b"syntax error near unexpected token \0" as *const u8 as *const libc::c_char,
				b"`newline'\0" as *const u8 as *const libc::c_char,
			);
			return 2 as libc::c_int;
		} else if ft_isalnum(*s.offset((*check).i as isize) as libc::c_int) != 0 {
			(*check).flag_redir = 0 as libc::c_int;
			(*check).flag_word = 1 as libc::c_int;
		}
		(*check).i = ((*check).i).wrapping_add(1);
		(*check).i;
	}
	return 0 as libc::c_int;
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
			eprint(
				b"%s %s\0" as *const u8 as *const libc::c_char,
				b"syntax error near unexpected token \0" as *const u8 as *const libc::c_char,
				b"`|'\0" as *const u8 as *const libc::c_char,
			);
			return 2 as libc::c_int;
		}
		if (*s.offset((*check).i as isize) == 0
			|| *s.offset((*check).i as isize) as libc::c_int == '|' as i32)
			&& ((*check).flag_redir != 0 || (*check).flag_word == 0)
		{
			eprint(
				b"%s %s\0" as *const u8 as *const libc::c_char,
				b"syntax error near unexpected token \0" as *const u8 as *const libc::c_char,
				b"`|'\0" as *const u8 as *const libc::c_char,
			);
			return 2 as libc::c_int;
		}
	}
	if *s.offset((*check).i as isize) as libc::c_int == '|' as i32 {
		(*check).ignore = 0 as libc::c_int != 0;
	}
	return 0 as libc::c_int;
}
unsafe extern "C" fn check_pipes_redirection_quotes(
	mut s: *const libc::c_char,
	mut input: *mut t_lexer,
	mut check: *mut s_check_pipes,
) -> libc::c_int {
	while (*check).i < (*input).len && !((*input).ignore).is_null() {
		if !*((*input).ignore).offset((*check).i as isize) {
			if inner_while_quotes(s, input, check) != 0 as libc::c_int
				|| inner_if_quotes(s, input, check) != 0 as libc::c_int
			{
				return 2 as libc::c_int;
			}
		}
		if *((*input).ignore).offset((*check).i as isize) {
			(*check).ignore = 1 as libc::c_int != 0;
			while *s.offset((*check).i as isize) as libc::c_int != 0
				&& *((*input).ignore).offset((*check).i as isize) as libc::c_int != 0
			{
				(*check).i = ((*check).i).wrapping_add(1);
				(*check).i;
			}
		} else {
			(*check).i = ((*check).i).wrapping_add(1);
			(*check).i;
		}
	}
	if (*check).flag_redir != 0 && !(*check).ignore {
		eprint(
			b"%s %s\0" as *const u8 as *const libc::c_char,
			b"syntax error near unexpected token \0" as *const u8 as *const libc::c_char,
			b"`newline'\0" as *const u8 as *const libc::c_char,
		);
		return 2 as libc::c_int;
	}
	return 0 as libc::c_int;
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
				as libc::c_int != 0
				&& *s.offset(((*check).i).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
					as libc::c_int == *s.offset((*check).i as isize) as libc::c_int
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
			eprint(
				b"%s %s\0" as *const u8 as *const libc::c_char,
				b"syntax error near unexpected token \0" as *const u8 as *const libc::c_char,
				b"`newline'\0" as *const u8 as *const libc::c_char,
			);
			return 2 as libc::c_int;
		} else if ft_isalnum(*s.offset((*check).i as isize) as libc::c_int) != 0 {
			(*check).flag_redir = 0 as libc::c_int;
			(*check).flag_word = 1 as libc::c_int;
		}
		(*check).i = ((*check).i).wrapping_add(1);
		(*check).i;
	}
	return 0 as libc::c_int;
}
#[no_mangle]
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
	check = {
		let mut init = s_check_pipes {
			flag_redir: 0 as libc::c_int,
			flag_word: 0 as libc::c_int,
			i: 0 as libc::c_int as size_t,
			ignore: 0 as libc::c_int != 0,
		};
		init
	};
	if *s as libc::c_int == '|' as i32
		|| *s.offset(((*input).len).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
			as libc::c_int
			== '|' as i32
	{
		eprint(
			b"%s %s\0" as *const u8 as *const libc::c_char,
			b"syntax error near unexpected token \0" as *const u8 as *const libc::c_char,
			b"`|'\0" as *const u8 as *const libc::c_char,
		);
		return 2 as libc::c_int;
	}
	if !(ft_strchr(
		b"<>\0" as *const u8 as *const libc::c_char,
		*s.offset(((*input).len).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
			as libc::c_int,
	))
	.is_null()
	{
		eprint(
			b"%s %s\0" as *const u8 as *const libc::c_char,
			b"syntax error near unexpected token \0" as *const u8 as *const libc::c_char,
			b"`newline'\0" as *const u8 as *const libc::c_char,
		);
		return 2 as libc::c_int;
	}
	if !((*input).ignore).is_null() {
		return check_pipes_redirection_quotes(s, input, &mut check);
	}
	while check.i < (*input).len {
		check = {
			let mut init = s_check_pipes {
				flag_redir: 0,
				flag_word: 0,
				i: check.i,
				ignore: false,
			};
			init
		};
		if inner_while_noquotes(s, &mut check) != 0 as libc::c_int {
			return 2 as libc::c_int;
		}
		if check.flag_word == 0 {
			eprint(
				b"%s %s\0" as *const u8 as *const libc::c_char,
				b"syntax error near unexpected token \0" as *const u8 as *const libc::c_char,
				b"`|'\0" as *const u8 as *const libc::c_char,
			);
			return 2 as libc::c_int;
		}
		if (*s.offset(check.i as isize) == 0
			|| *s.offset(check.i as isize) as libc::c_int == '|' as i32)
			&& (check.flag_redir != 0 || check.flag_word == 0)
		{
			eprint(
				b"%s %s\0" as *const u8 as *const libc::c_char,
				b"syntax error near unexpected token \0" as *const u8 as *const libc::c_char,
				b"`|'\0" as *const u8 as *const libc::c_char,
			);
			return 2 as libc::c_int;
		}
		while *s.offset(check.i as isize) as libc::c_int != 0
			&& *s.offset(check.i as isize) as libc::c_int == '|' as i32
		{
			check.i = (check.i).wrapping_add(1);
			check.i;
		}
	}
	if check.flag_redir != 0 {
		eprint(
			b"%s %s\0" as *const u8 as *const libc::c_char,
			b"syntax error near unexpected token \0" as *const u8 as *const libc::c_char,
			b"`newline'\0" as *const u8 as *const libc::c_char,
		);
		return 2 as libc::c_int;
	}
	return 0 as libc::c_int;
}
