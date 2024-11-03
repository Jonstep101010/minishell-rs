use ::libc;
extern "C" {
	fn ft_memcpy(dst: *mut libc::c_void, src: *const libc::c_void, n: size_t) -> *mut libc::c_void;
	fn ft_strlen(str: *const libc::c_char) -> size_t;
	fn str_cchr(s: *const libc::c_char, c: libc::c_char) -> libc::c_int;
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
#[no_mangle]
pub unsafe extern "C" fn count_number(mut s: *const libc::c_char, mut input: *mut t_lexer) {
	let data: t_lexer = {
		let mut init = s_lexer {
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
			ignore: 0 as *mut bool,
			len: ft_strlen(s) as libc::c_int as size_t,
			lexer: 0 as libc::c_int,
			result: 0 as libc::c_int != 0,
		};
		init
	};
	ft_memcpy(
		input as *mut libc::c_void,
		&data as *const t_lexer as *const libc::c_void,
		::core::mem::size_of::<t_lexer>() as libc::c_ulong,
	);
}
