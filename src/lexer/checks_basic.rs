use ::libc;
extern "C" {
	fn check_pipes_redirection(s: *const libc::c_char, input: *mut t_lexer) -> libc::c_int;
	fn count_number(s: *const libc::c_char, input: *mut s_lexer);
	fn bool_arr_zeroing(len: size_t) -> *mut bool;
	fn range_ignore(s: *const libc::c_char, ignore: *mut bool, c: libc::c_uchar);
	fn free(_: *mut libc::c_void);
	fn ft_calloc(nitems: size_t, size: size_t) -> *mut libc::c_void;
	fn ft_strlen(str: *const libc::c_char) -> size_t;
	fn eprint_single(fmt: *const libc::c_char, _: ...);
}
pub type size_t = libc::c_ulong;
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
unsafe extern "C" fn ignore_quotes(
	mut s: *const libc::c_char,
	mut input: *mut s_lexer,
) -> libc::c_int {
	if s.is_null() || input.is_null() {
		return 1 as libc::c_int;
	}
	(*input).ignore = bool_arr_zeroing(ft_strlen(s));
	range_ignore(s, (*input).ignore, '"' as i32 as libc::c_uchar);
	range_ignore(s, (*input).ignore, '\'' as i32 as libc::c_uchar);
	return 0 as libc::c_int;
}
unsafe extern "C" fn check_quotes(
	mut s: *const libc::c_char,
	mut input: *mut s_lexer,
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
	return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lexer_checks_basic(mut s: *const libc::c_char) -> *mut t_lexer {
	let mut input: *mut t_lexer = 0 as *mut t_lexer;
	input = ft_calloc(
		::core::mem::size_of::<t_lexer>() as libc::c_ulong,
		1 as libc::c_int as size_t,
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
	return input;
}
