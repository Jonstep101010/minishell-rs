mod check_pipes;
mod checks_basic;
mod lexer_support;

use crate::{
	size_t, t_shell,
	tokenizer::{build_tokens::tokenize, destroy_tokens::destroy_all_tokens},
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
#[unsafe(no_mangle)]
pub unsafe fn run(mut shell: *mut t_shell, mut trimmed_line: *const libc::c_char) -> libc::c_int {
	if *trimmed_line == 0 {
		return 0 as libc::c_int;
	}
	let mut lex = lexer_checks_basic(trimmed_line);
	if !(*lex).result {
		(*shell).exit_status = (*lex).lexer as u8;
		free(lex as *mut libc::c_void);
		return 1 as libc::c_int;
	}
	free(lex as *mut libc::c_void);
	(*shell).token = tokenize(shell, trimmed_line) as *mut crate::t_token;
	if ((*shell).token).is_null() {
		return -(1 as libc::c_int);
	}
	if ((*(*shell).token).cmd_args).is_null() {
		destroy_all_tokens(shell);
		return -(1 as libc::c_int);
	}
	0 as libc::c_int
}

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::{fixture, rstest};

	unsafe fn lexer_mock(trimmed_line: String) -> i32 {
		assert!(!trimmed_line.is_empty());
		// if *trimmed_line == 0 {
		// 	return 0;
		// }
		let cstr = std::ffi::CString::new(trimmed_line).unwrap();
		let mut lex = lexer_checks_basic(cstr.as_ptr());
		if !(*lex).result {
			libc::free(lex as *mut libc::c_void);
			return 1;
		}
		libc::free(lex as *mut libc::c_void);
		0
	}
	#[rstest]
	#[case("echo \"|\"")]
	#[case("echo '|'")]
	#[case("\"|\"")]
	#[case("'|'")]
	#[case("echo \"Hello || World | hello\"")]
	#[case("echo Hello | World | hello")]
	#[case("ls \"||\"| grep file '||'| wc -l")]
	#[case("ls '|' hello '>>'")]
	#[case("ls < infile")]
	#[case("ls << infile")]
	#[case("< infile")]
	#[case("<< infile")]
	#[case("ls > outfile")]
	#[case("ls >> outfile")]
	#[case(">> outfile")]
	#[case("echo \"|\"")]
	#[case("echo '|'")]
	#[case("\"|\"")]
	#[case("'|'")]
	#[case("echo \"Hello || World | hello\"")]
	#[case("echo Hello | World | hello")]
	#[case("echo Hello | World | hello")]
	#[case("ls \"||\"| grep file '||'| wc -l")]
	#[case("   < infile |  < infiel| cat")]
	#[case("echo hello || cat")]
	#[case("hello | hello")]
	#[case("hello|hello")]
	#[case("echo Hello | World")]
	#[case("ls | grep file | wc -l")]
	#[case("hello || hello")]
	#[case("> infile | cat")]
	#[case("cat << delim | > tmp_out")]
	#[case("<infile | cat")]
	#[case("cat <\"./test_files/infile\" | echo hi")]
	#[case("cat << \"$USER\"")]
	#[case("cat << \"$US\"E\"R\"")]
	#[case(">> \"$USER'$USER'\"")]
	#[case("\"$USER'$USER'\" | echo")]
	#[case("\"$USER'$USER'\" | \"echo\"")]
	#[case("$USER'$USER' | \"echo\"")]
	#[case("echo \"This is a test\"")]
	#[case("echo \"Hello, World!\"")]
	#[case("\n")]
	#[case("\n \n")]
	#[case("\n	\n \n")]
	#[case("\n\n\n\n")]
	#[case("\n\n\n\n\n")]
	#[case("\n\n\n\n\n\n")]
	#[case("\n\n\n\n\n\n\n")]
	#[case("\n\n\n\n\n\n\n\n")]
	#[case("\n\n\n\n\n\n\n\n\n")]
	#[case("\n\n\n\n\n\\n\n\n\n\n")]
	#[case("echo > outfile")]
	#[case("> outfile")]
	#[case("< outfile")]
	#[case("cat << delim | > outfile")]
	#[fixture]
	fn lexer_success(#[case] input: &str) {
		unsafe {
			assert_eq!(0, lexer_mock(input.to_string()));
		}
	}
	#[rstest]
	#[case("ls > outfile >")]
	#[case("ls < infile <")]
	#[case(">")]
	#[case(">>")]
	#[case(">>>")]
	#[case("<")]
	#[case("<<")]
	#[case("<<<")]
	#[case("< < infile")]
	#[case("echo |")]
	#[case("ls << infile >>")]
	#[case("hello ||")]
	#[case("hello |")]
	#[case("hello | hello | hello |")]
	#[case("     ||")]
	#[case("           |    ")]
	#[case("			|    ")]
	#[case("        | |")]
	#[case("   <  |  < |")]
	#[case("    |   |")]
	#[case("   < |||")]
	#[case("      |")]
	#[case("||")]
	#[case("||       ")]
	#[case("|")]
	#[case("           |")]
	#[case("			|")]
	#[case("| |")]
	#[case("|   |")]
	#[case("|||")]
	#[case("      |")]
	#[case(">|")]
	#[case("<|")]
	#[case("> |")]
	#[case("< |")]
	#[case("echo Hello, World!\"")]
	#[case("echo Hello, World!\"")]
	#[case("echo \"Hello, World!")]
	#[case("echo \'Hello, World!")]
	#[case("echo 'Hello, World!")]
	#[case("'\"'")]
	#[case("echo \"'Hello, World!\"")]
	#[case("\"'\"")]
	#[case("< < <")]
	#[case("> > >")]
	#[case("> > >")]
	#[case("> tmpfile > midfile >")]
	#[fixture]
	fn lexer_failure(#[case] input: &str) {
		unsafe {
			assert_eq!(1, lexer_mock(input.to_string()));
		}
	}
}
