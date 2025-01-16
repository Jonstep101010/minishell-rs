mod check_pipes;
mod checks_basic;
mod lexer_support;

use crate::{size_t, t_shell};
use ::libc::{self, free};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct t_lexer<'a> {
	pub singlequotes: i32,
	pub doublequotes: i32,
	pub open_curly_brackets: i32,
	pub close_curly_brackets: i32,
	pub open_square_brackets: i32,
	pub close_square_brackets: i32,
	pub open_parentheses: i32,
	pub close_parentheses: i32,
	pub redir_greater: i32,
	pub redir_smaller: i32,
	pub pipes: i32,
	pub ignore: *mut bool,
	pub len: usize,
	pub lexer: i32,
	pub result: bool,
	pub trimmed_line: &'a str,
	pub cstring: std::ffi::CString,
}

impl<'a> t_lexer<'a> {
	pub fn new(trimmed_line: &'a str) -> Self {
		let mut lexer = t_lexer {
			singlequotes: 0,
			doublequotes: 0,
			open_curly_brackets: 0,
			close_curly_brackets: 0,
			open_square_brackets: 0,
			close_square_brackets: 0,
			open_parentheses: 0,
			close_parentheses: 0,
			redir_greater: 0,
			redir_smaller: 0,
			pipes: 0,
			ignore: std::ptr::null_mut::<bool>(),
			len: 0,
			lexer: 0,
			result: false,
			trimmed_line,
			cstring: std::ffi::CString::new(trimmed_line).unwrap(),
		};
		// declaring these as enum variants would be better ; |
		for &c in trimmed_line.as_bytes() {
			match c {
				b'\'' => lexer.singlequotes += 1,
				b'"' => lexer.doublequotes += 1,
				b'{' => lexer.open_curly_brackets += 1,
				b'}' => lexer.close_curly_brackets += 1,
				b'[' => lexer.open_square_brackets += 1,
				b']' => lexer.close_square_brackets += 1,
				b'(' => lexer.open_parentheses += 1,
				b')' => lexer.close_parentheses += 1,
				b'<' => lexer.redir_smaller += 1,
				b'>' => lexer.redir_greater += 1,
				b'|' => lexer.pipes += 1,
				_ => {}
			}
		}
		lexer.ignore = std::ptr::null_mut::<bool>();
		lexer.len = lexer.cstring.count_bytes();
		lexer
	}
}
use checks_basic::lexer_checks_basic;
#[unsafe(no_mangle)]
pub unsafe fn run(shell: &mut t_shell, trimmed_line: &str) -> i32 {
	if trimmed_line.is_empty() {
		return 0;
	}
	let mut lexer = t_lexer::new(trimmed_line);
	let mut lex = lexer_checks_basic(cstring.as_ptr());
	if !(*lex).result {
		shell.exit_status = (*lex).lexer as u8;
		free(lex as *mut libc::c_void);
		return 1;
	}
	free(lex as *mut libc::c_void);
	0
}

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::{fixture, rstest};

	unsafe fn lexer_mock(trimmed_line: String) -> i32 {
		assert!(!trimmed_line.is_empty());
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
