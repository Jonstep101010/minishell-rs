use crate::{eprint_msh, t_shell};
use ::libc;
use ::libc::free;
use libft_rs::{ft_isalnum::ft_isalnum, ft_strchr::ft_strchr};
use libutils_rs::src::char::is_something::ft_isspace;

#[derive(Clone)]
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
	pub len_nul: usize,
	pub trimmed_line: &'a str,
	pub cstring: std::ffi::CString,
	pub len: usize,
}

struct s_check_pipes {
	pub flag_redir: i32,
	pub flag_word: i32,
	pub i: usize,
	pub ignore: bool,
}

impl<'a> t_lexer<'a> {
	///
	/// returns ok or err, meaning newline
	unsafe fn inner_while_quotes(&self, mut check: &mut s_check_pipes) -> Result<(), ()> {
		let s = self.cstring.as_ptr();
		check.flag_word = 0 as libc::c_int;
		check.flag_redir = 0 as libc::c_int;
		while *s.add(check.i) as libc::c_int != 0
			&& *s.add(check.i) as libc::c_int != '|' as i32
			&& !*((*self).ignore).add(check.i)
		{
			if !(ft_strchr(
				b"><\0" as *const u8 as *const libc::c_char,
				*s.add(check.i) as libc::c_int,
			))
			.is_null() && (check.flag_redir == 0
				|| *s.offset((check.i).wrapping_sub(1) as isize) as libc::c_int != 0
					&& *s.offset((check.i).wrapping_sub(1) as isize) as libc::c_int
						== *s.add(check.i) as libc::c_int)
			{
				check.flag_redir = 1 as libc::c_int;
			} else if !(ft_strchr(
				b"><\0" as *const u8 as *const libc::c_char,
				*s.add(check.i) as libc::c_int,
			))
			.is_null()
			{
				eprint_msh!("syntax error near unexpected token `newline'");
				return Err(());
			} else if ft_isalnum(*s.add(check.i) as libc::c_int) != 0 {
				check.flag_redir = 0 as libc::c_int;
				check.flag_word = 1 as libc::c_int;
			}
			check.i = (check.i).wrapping_add(1);
		}
		Ok(())
	}
	unsafe fn inner_if_quotes(&self, mut check: &mut s_check_pipes) -> Result<(), ()> {
		let s = self.cstring.as_ptr();
		if !*((*self).ignore).add(check.i)
			&& *s.add(check.i) as libc::c_int == '|' as i32
			&& !check.ignore
		{
			if check.flag_word == 0 {
				eprint_msh!("syntax error near unexpected token `|'");
				return Err(());
			}
			if (*s.add(check.i) == 0 || *s.add(check.i) as libc::c_int == '|' as i32)
				&& (check.flag_redir != 0 || check.flag_word == 0)
			{
				eprint_msh!("syntax error near unexpected token `|'");
				return Err(());
			}
		}
		if *s.add(check.i) as libc::c_int == '|' as i32 {
			check.ignore = 0 as libc::c_int != 0;
		}
		Ok(())
	}
	unsafe fn check_pipes_redirection_quotes(&mut self) -> Result<(), i32> {
		let s = self.cstring.as_ptr();
		let mut check: s_check_pipes = s_check_pipes {
			flag_redir: 0,
			flag_word: 0,
			i: 0,
			ignore: false,
		};
		if !(self.ignore.is_null()) {
			while (check).i < (*self).len_nul {
				if !*((*self).ignore).add(check.i) {
					if self.inner_while_quotes(&mut check).is_err()
						|| self.inner_if_quotes(&mut check).is_err()
					{
						return Err(2);
					}
				}
				if *((*self).ignore).add(check.i) {
					check.ignore = true;
					while *s.add(check.i) as libc::c_int != 0
						&& *((*self).ignore).add(check.i) as libc::c_int != 0
					{
						check.i = (check.i).wrapping_add(1);
					}
				} else {
					check.i = (check).i.wrapping_add(1);
				}
			}
		}
		if check.flag_redir != 0 && !check.ignore {
			eprint_msh!("syntax error near unexpected token `newline'");
			return Err(2);
		}
		Ok(())
	}
	unsafe fn inner_while_noquotes(&self, mut check: &mut s_check_pipes) -> Result<(), ()> {
		let s = self.trimmed_line.as_ptr();
		while check.i < self.len_nul - 1 && *s.add(check.i) as libc::c_int != '|' as i32 {
			if !(ft_strchr(
				b"><\0" as *const u8 as *const libc::c_char,
				*s.add(check.i) as libc::c_int,
			))
			.is_null() && (check.flag_redir == 0
				|| *s.offset((check.i).wrapping_sub(1) as isize) as libc::c_int != 0
					&& *s.offset((check.i).wrapping_sub(1) as isize) as libc::c_int
						== *s.add(check.i) as libc::c_int
					&& (*s.offset((check.i).wrapping_sub(2) as isize) == 0
						|| ft_isspace(*s.offset((check.i).wrapping_sub(2) as isize) as libc::c_int)
							!= 0))
			{
				check.flag_redir = 1 as libc::c_int;
			} else if !(ft_strchr(
				b"><\0" as *const u8 as *const libc::c_char,
				*s.add(check.i) as libc::c_int,
			))
			.is_null()
			{
				eprint_msh!("syntax error near unexpected token `newline'");
				return Err(());
			} else if ft_isalnum(*s.add(check.i) as libc::c_int) != 0 {
				check.flag_redir = 0 as libc::c_int;
				check.flag_word = 1 as libc::c_int;
			}
			check.i = (check.i).wrapping_add(1);
		}
		Ok(())
	}

	unsafe fn check_pipes_redirection(&mut self) -> Result<(), i32> {
		if self.trimmed_line.starts_with('|') || self.trimmed_line.ends_with('|') {
			eprint_msh!("syntax error near unexpected token `|'");
			return Err(2);
		}
		if self.trimmed_line.ends_with(|c| "<>".contains(c)) {
			eprint_msh!("syntax error near unexpected token `newline'");
			return Err(2);
		}
		if !((*self).ignore).is_null() {
			return self.check_pipes_redirection_quotes();
		}
		let mut check: s_check_pipes = s_check_pipes {
			flag_redir: 0,
			flag_word: 0,
			i: 0,
			ignore: false,
		};
		while check.i < (*self).len_nul - 1 {
			check = {
				s_check_pipes {
					flag_redir: 0,
					flag_word: 0,
					i: check.i,
					ignore: false,
				}
			};
			if self.inner_while_noquotes(&mut check).is_err() {
				return Err(2);
			}
			if check.flag_word == 0 {
				eprint_msh!("syntax error near unexpected token `|'");
				return Err(2);
			}
			if (self.trimmed_line.as_bytes()[check.i] == b'|')
				&& (check.flag_redir != 0 || check.flag_word == 0)
			{
				eprint_msh!("syntax error near unexpected token `|'");
				return Err(2);
			}
			while check.i < self.len_nul - 1 && self.trimmed_line.as_bytes()[check.i] == b'|' {
				check.i = (check.i).wrapping_add(1);
			}
		}
		if check.flag_redir != 0 {
			crate::eprint_msh!("syntax error near unexpected token `newline'");
			return Err(2);
		}
		Ok(())
	}
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
			len_nul: 0,
			trimmed_line,
			cstring: std::ffi::CString::new(trimmed_line).unwrap(),
			len: trimmed_line.as_bytes().len(),
		};
		// declaring these as enum variants would be better ; |
		for &c in trimmed_line.as_bytes() {
			// replaces count_number
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
		lexer.len_nul = lexer.cstring.count_bytes();
		lexer
	}
	///
	/// uses `bool_array`
	unsafe fn ignore_quotes(&mut self) {
		// if s.is_null() || input.is_null() {
		// 	return 1 as libc::c_int;
		// }
		self.ignore = crate::utils::bool_array::bool_arr_zeroing(self.len_nul as u64);
		crate::utils::bool_array::range_ignore(
			self.cstring.as_ptr(),
			self.ignore,
			'"' as i32 as libc::c_uchar,
		);
		crate::utils::bool_array::range_ignore(
			self.cstring.as_ptr(),
			self.ignore,
			'\'' as i32 as libc::c_uchar,
		);
	}
	fn check_quotes(&mut self) -> Result<i32, i32> {
		if self.singlequotes == 1 as libc::c_int {
			eprintln!("syntax error near unexpected token '''");
			return Err(1);
		}
		if self.doublequotes == 1 as libc::c_int {
			eprintln!("syntax error near unexpected token '\"'");
			return Err(1);
		}
		if self.singlequotes % 2 as libc::c_int != 0 || self.doublequotes % 2 as libc::c_int != 0 {
			eprintln!("error: quotes not closed");
			return Err(1);
		}
		if self.singlequotes > 0 || self.doublequotes > 0 {
			unsafe {
				self.ignore_quotes();
			}
		}
		Ok(0)
	}
	fn checks_basic(&mut self) -> Result<i32, i32> {
		if self.check_quotes().is_err() {
			unsafe {
				free(self.ignore as *mut libc::c_void);
			}
			return Err(0);
		}
		if self.pipes != 0 || self.redir_greater != 0 || self.redir_smaller != 0 {
			match unsafe { self.check_pipes_redirection() } {
				// free the ignore
				Err(_e) => {
					// map error printing in future
					unsafe { free(self.ignore as *mut libc::c_void) };
					return Err(2);
				}
				Ok(_) => {}
			}
			// 	return Err(self.lexer);
		}
		unsafe { free(self.ignore as *mut libc::c_void) };
		Ok(0)
	}
}

pub fn run(shell: &mut t_shell, trimmed_line: &str) -> i32 {
	if trimmed_line.is_empty() {
		return 0;
	}
	let mut lexer = t_lexer::new(trimmed_line);
	match t_lexer::checks_basic(&mut lexer) {
		Ok(0) => 0,
		Err(status_result) => {
			shell.exit_status = status_result as u8;
			1
		}
		Ok(_) => {
			unreachable!("this should contain the lexing result!")
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::{fixture, rstest};

	unsafe fn lexer_mock(trimmed_line: String) -> i32 {
		assert!(!trimmed_line.is_empty());
		// let cstr = std::ffi::CString::new(trimmed_line).unwrap();
		// let mut lex = lexer_checks_basic(cstr.as_ptr());
		// if !(*lex).result {
		// 	libc::free(lex as *mut libc::c_void);
		// 	return 1;
		// }
		// libc::free(lex as *mut libc::c_void);
		let mut lex = t_lexer::new(&trimmed_line);
		if t_lexer::checks_basic(&mut lex).is_err() {
			1
		} else {
			0
		}
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
