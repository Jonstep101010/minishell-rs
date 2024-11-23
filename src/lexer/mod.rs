use crate::{
	environment::export_env::update_exit_status,
	size_t, t_shell,
	tokenizer::{build_tokens::tokenize, destroy_tokens::destroy_all_tokens},
	utils::get_input::get_input,
};
use ::libc::{self, free};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct t_lexer {
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
use checks_basic::lexer_checks_basic;
#[no_mangle]
pub unsafe extern "C" fn run(
	mut shell: *mut t_shell,
	mut trimmed_line: *const libc::c_char,
) -> libc::c_int {
	if *trimmed_line == 0 {
		get_input(std::ptr::null_mut::<libc::c_char>());
		return 0 as libc::c_int;
	}
	let mut lex = lexer_checks_basic(trimmed_line);
	if !(*lex).result {
		update_exit_status(shell, (*lex).lexer);
		get_input(std::ptr::null_mut::<libc::c_char>());
		free(lex as *mut libc::c_void);
		return 1 as libc::c_int;
	}
	free(lex as *mut libc::c_void);
	(*shell).token = tokenize(shell, trimmed_line) as *mut crate::t_token;
	get_input(std::ptr::null_mut::<libc::c_char>());
	if ((*shell).token).is_null() {
		return -(1 as libc::c_int);
	}
	if ((*(*shell).token).cmd_args).is_null() {
		destroy_all_tokens(shell);
		return -(1 as libc::c_int);
	}
	0 as libc::c_int
}

mod check_pipes;
mod checks_basic;
mod lexer_support;
