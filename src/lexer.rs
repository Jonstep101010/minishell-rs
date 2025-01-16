use crate::eprint_msh;

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
	pub ignore: Option<Box<[bool]>>, // for this purpose same as Vec<bool>
	pub len_nul: usize,
	pub trimmed_line: &'a str,
	pub cstring: std::ffi::CString,
}

impl<'a> t_lexer<'a> {
	fn check_pipes_redirection_quotes(&mut self) -> Result<(), i32> {
		if self.ignore.is_some() {
			let mut flag_redir = 0;
			let mut flag_word = 0;
			let mut check_ignore = false;
			let mut i = 0;
			let bytes = self.cstring.as_bytes_with_nul();
			while i < self.len_nul - 1 {
				if !(self.ignore.as_ref().unwrap())[i] && {
					flag_word = 0;
					flag_redir = 0;
					while i < self.len_nul - 1
						&& bytes[i] != b'|'
						&& !(self.ignore.as_ref().unwrap())[i]
					{
						if (bytes[i] == b'>' || bytes[i] == b'<')
							&& (flag_redir == 0 || (i > 0) && bytes[i - 1] == bytes[i])
						{
							flag_redir = 1;
						} else if bytes[i] == b'<' || bytes[i] == b'>' {
							eprint_msh!("syntax error near unexpected token `newline'");
							return Err(2);
						} else if bytes[i].is_ascii_alphanumeric() {
							flag_redir = 0;
							flag_word = 1;
						}
						i += 1;
					}
					false
				} || {
					// inner if quotes
					if !((self.ignore.as_ref()).unwrap()[i]) && bytes[i] == b'|' && !check_ignore {
						if flag_word == 0 {
							eprint_msh!("syntax error near unexpected token `|'");
							return Err(2);
						}
						if (bytes[i] == b'|') && (flag_redir != 0 || flag_word == 0) {
							eprint_msh!("syntax error near unexpected token `|'");
							return Err(2);
						}
					}
					if bytes[i] == b'|' {
						check_ignore = false;
					}
					false
				} {}
				if (self.ignore.as_ref()).unwrap()[i] {
					check_ignore = true;
					while i < self.len_nul - 1 && (self.ignore.as_ref()).unwrap()[i] == true {
						i = (i).wrapping_add(1);
					}
				} else {
					i = i.wrapping_add(1);
				}
			}
			if flag_redir != 0 && !check_ignore {
				eprint_msh!("syntax error near unexpected token `newline'");
				return Err(2);
			}
		}
		Ok(())
	}

	fn check_pipes_redirection(&mut self) -> Result<(), i32> {
		if self.trimmed_line.starts_with('|') || self.trimmed_line.ends_with('|') {
			eprint_msh!("syntax error near unexpected token `|'");
			return Err(2);
		}
		if self.trimmed_line.ends_with(|c| "<>".contains(c)) {
			eprint_msh!("syntax error near unexpected token `newline'");
			return Err(2);
		}
		if (self.ignore).is_some() {
			return self.check_pipes_redirection_quotes();
		}
		let bytes = self.trimmed_line.as_bytes();
		let mut i = 0;
		let mut flag_redir = 0;
		while i < self.len_nul - 1 {
			flag_redir = 0;
			let mut flag_word = 0;
			while i < self.len_nul - 1 && bytes[i] != b'|' {
				if (bytes[i] == b'>' || bytes[i] == b'<')
					&& (flag_redir == 0
						|| i > 0
							&& bytes[(i).wrapping_sub(1)] == bytes[i]
							&& (i == 1 || bytes[(i).wrapping_sub(2)].is_ascii_whitespace()))
				{
					flag_redir = 1;
				} else if bytes[i] == b'>' || bytes[i] == b'<' {
					eprint_msh!("syntax error near unexpected token `newline'");
					return Err(2);
				} else if bytes[i].is_ascii_alphanumeric() {
					flag_redir = 0;
					flag_word = 1;
				}
				i += 1;
			}
			if flag_word == 0 {
				eprint_msh!("syntax error near unexpected token `|'");
				return Err(2);
			}
			if (bytes[i] == b'|') && (flag_redir != 0 || flag_word == 0) {
				eprint_msh!("syntax error near unexpected token `|'");
				return Err(2);
			}
			while i < self.len_nul - 1 && bytes[i] == b'|' {
				i = (i).wrapping_add(1);
			}
		}
		if flag_redir != 0 {
			crate::eprint_msh!("syntax error near unexpected token `newline'");
			return Err(2);
		}
		Ok(())
	}
	fn new(trimmed_line: &'a str) -> Self {
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
			ignore: None,
			len_nul: 0,
			trimmed_line,
			cstring: std::ffi::CString::new(trimmed_line).unwrap(),
		};
		// declaring these as enum variants would be better ; |
		for &c in lexer.cstring.as_bytes() {
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
	fn check_quotes(&mut self) -> Result<i32, i32> {
		if self.singlequotes == 1 {
			eprintln!("syntax error near unexpected token '''");
			return Err(1);
		}
		if self.doublequotes == 1 {
			eprintln!("syntax error near unexpected token '\"'");
			return Err(1);
		}
		if self.singlequotes % 2 != 0 || self.doublequotes % 2 != 0 {
			eprintln!("error: quotes not closed");
			return Err(1);
		}
		if self.singlequotes > 0 || self.doublequotes > 0 {
			// ignore_quotes
			use crate::utils::bool_array::{BoolArray, bool_arr_zeroing_box};
			self.ignore = Some(bool_arr_zeroing_box(self.len_nul));
			BoolArray::range_ignore(
				self.cstring.as_bytes_with_nul(),
				self.ignore.as_mut().unwrap(),
				b'"',
			);
			BoolArray::range_ignore(
				self.cstring.as_bytes_with_nul(),
				self.ignore.as_mut().unwrap(),
				b'\'',
			);
		}
		Ok(0)
	}
	fn checks_basic(&mut self) -> Result<i32, i32> {
		self.check_quotes()?;
		if self.pipes != 0 || self.redir_greater != 0 || self.redir_smaller != 0 {
			if let Err(_e) = self.check_pipes_redirection() {
				// map error printing in future
				return Err(2);
			}
		}
		Ok(0)
	}
	pub fn run(trimmed_line: &str) -> Result<i32, i32> {
		let mut lexer = t_lexer::new(trimmed_line);
		t_lexer::checks_basic(&mut lexer)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::{fixture, rstest};

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
		assert_eq!(Ok(0), t_lexer::run(input));
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
		assert!(t_lexer::run(input).is_err());
	}
}
