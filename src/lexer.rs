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
pub unsafe extern "C" fn run(
	mut shell: *mut t_shell,
	mut trimmed_line: *const libc::c_char,
) -> libc::c_int {
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
	#[fixture]
	fn lexer_failure(#[case] input: &str) {
		unsafe {
			assert_eq!(1, lexer_mock(input.to_string()));
		}
	}
	#[test]
	fn test_redir_pipes_mix() {
		let inputs_ok = [
			"> infile | cat",
			"cat << delim | > tmp_out",
			"<infile | cat",
		];
		for input in inputs_ok {
			unsafe {
				assert_eq!(0, lexer_mock(input.to_string()));
			}
		}
		let inputs_err = [">|", "<|", "> |", "< |"];
		for input in inputs_err {
			unsafe {
				assert_eq!(1, lexer_mock(input.to_string()));
			}
		}
	}
	#[test]
	fn test_redir_quotes() {
		unsafe {
			assert_eq!(
				0,
				lexer_mock("cat <\"./test_files/infile\" | echo hi".to_string())
			);
			assert_eq!(0, lexer_mock("cat << \"$USER\"".to_string()));
			assert_eq!(0, lexer_mock("cat << \"$US\"E\"R\"".to_string()));
			assert_eq!(0, lexer_mock(">> \"$USER'$USER'\"".to_string()));
			assert_eq!(0, lexer_mock("\"$USER'$USER'\" | echo".to_string()));
			assert_eq!(0, lexer_mock("\"$USER'$USER'\" | \"echo\"".to_string()));
			assert_eq!(0, lexer_mock("$USER'$USER' | \"echo\"".to_string()));
		}
	}
	#[test]
	fn test_works_with_quotes() {
		let inputs_ok = ["echo \"This is a test\"", "echo \"Hello, World!\""];
		for input in inputs_ok {
			unsafe {
				assert_eq!(0, lexer_mock(input.to_string()));
			}
		}
		let inputs_err = [
			"echo Hello, World!\"",
			"echo Hello, World!\"",
			"echo \"Hello, World!",
			"echo \'Hello, World!",
			"echo 'Hello, World!",
			"'\"'",
			"echo \"'Hello, World!\"",
			"\"'\"",
		];
		for input in inputs_err {
			unsafe {
				assert_eq!(1, lexer_mock(input.to_string()));
			}
		}
	}
	#[test]
	fn test_works_with_newlines() {
		let inputs_ok = [
			"\n",
			"\n \n",
			"\n	\n \n",
			"\n\n\n\n",
			"\n\n\n\n\n",
			"\n\n\n\n\n\n",
			"\n\n\n\n\n\n\n",
			"\n\n\n\n\n\n\n\n",
			"\n\n\n\n\n\n\n\n\n",
			"\n\n\n\n\n\\n\n\n\n\n",
		];
		for input in inputs_ok {
			unsafe {
				assert_eq!(0, lexer_mock(input.to_string()));
			}
		}
	}
	#[test]
	fn test_redirs() {
		let inputs_ok = [
			"echo > outfile",
			"> outfile",
			"< outfile",
			"cat << delim | > outfile",
		];
		for input in inputs_ok {
			unsafe {
				assert_eq!(0, lexer_mock(input.to_string()));
			}
		}
		let inputs_err = ["< < <", "> > >", "> > >", "> tmpfile > midfile >"];
		for input in inputs_err {
			unsafe {
				assert_eq!(1, lexer_mock(input.to_string()));
			}
		}
	}
}

mod check_pipes;
mod checks_basic;
mod lexer_support;
