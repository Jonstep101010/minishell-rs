use ::libc;
use libft_rs::{ft_memcpy::ft_memcpy, ft_strlen::ft_strlen};
use libutils_rs::src::string::str_cchr::str_cchr;

use super::t_lexer;

#[unsafe(no_mangle)]
pub unsafe fn count_number(mut s: *const libc::c_char, mut input: *mut t_lexer) {
	let data: t_lexer = {
		t_lexer {
			singlequotes: str_cchr(s, '\'' as i32 as libc::c_char),
			doublequotes: str_cchr(s, '"' as i32 as libc::c_char),
			open_curly_brackets: str_cchr(s, '{' as i32 as libc::c_char),
			close_curly_brackets: str_cchr(s, '}' as i32 as libc::c_char),
			open_square_brackets: str_cchr(s, '[' as i32 as libc::c_char),
			close_square_brackets: str_cchr(s, ']' as i32 as libc::c_char),
			open_parentheses: str_cchr(s, '(' as i32 as libc::c_char),
			close_parentheses: str_cchr(s, ')' as i32 as libc::c_char),
			redir_greater: str_cchr(s, '>' as i32 as libc::c_char),
			redir_smaller: str_cchr(s, '<' as i32 as libc::c_char),
			pipes: str_cchr(s, '|' as i32 as libc::c_char),
			ignore: std::ptr::null_mut::<bool>(),
			len: ft_strlen(s) as libc::c_int as crate::size_t,
			lexer: 0 as libc::c_int,
			result: 0 as libc::c_int != 0,
		}
	};
	ft_memcpy(
		input as *mut libc::c_void,
		&data as *const t_lexer as *const libc::c_void,
		::core::mem::size_of::<t_lexer>() as libc::c_ulong,
	);
}
