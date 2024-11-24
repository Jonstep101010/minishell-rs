use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Token {
	Word(String),
	Pipe,
	Redirection(Redirection),
	EndOfInput,
}

#[derive(Debug, PartialEq)]
pub enum Redirection {
	Input,
	Output,
	Append,
	HereDoc,
}

#[derive(Debug)]
pub struct Lexer<'a> {
	input: Peekable<Chars<'a>>,
}

#[derive(Debug, PartialEq)]
pub enum LexerError {
	UnexpectedToken(&'static str),
}

impl std::fmt::Display for LexerError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			LexerError::UnexpectedToken(token) => write!(f, "near unexpected token: `{}'", token),
		}
	}
}

impl<'a> Lexer<'a> {
	pub fn new(input: &'a str) -> Self {
		Lexer {
			input: input.chars().peekable(),
		}
	}

	// TODO: handle trailing pipes
	// TODO: handle >, <
	// TODO: handle logic operators (&&, ||)
	pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
		let mut tokens = Vec::new();
		let mut last_was_pipe = false;

		while let Some(&ch) = self.input.peek() {
			match ch {
				' ' | '\t' | '\n' => {
					self.input.next();
				}
				'|' => {
					if last_was_pipe {
						return Err(LexerError::UnexpectedToken("|"));
					}
					self.input.next();
					tokens.push(Token::Pipe);
					last_was_pipe = true;
				}
				'<' | '>' => {
					tokens.push(self.parse_redirection());
					last_was_pipe = false;
				}
				'\'' | '"' => {
					tokens.push(Token::Word(self.parse_quoted()));
					last_was_pipe = false;
				}
				_ => {
					tokens.push(Token::Word(self.parse_word()));
					last_was_pipe = false;
				}
			}
		}

		tokens.push(Token::EndOfInput);
		Ok(tokens)
	}

	fn parse_word(&mut self) -> String {
		let mut word = String::new();
		while let Some(&ch) = self.input.peek() {
			if ch.is_whitespace() || ch == '|' || ch == '<' || ch == '>' {
				break;
			}
			word.push(ch);
			self.input.next();
		}
		word
	}

	fn parse_quoted(&mut self) -> String {
		let quote = self.input.next().unwrap();
		let mut quoted = String::new();
		while let Some(&ch) = self.input.peek() {
			if ch == quote {
				self.input.next();
				break;
			}
			quoted.push(ch);
			self.input.next();
		}
		quoted
	}

	fn parse_redirection(&mut self) -> Token {
		let first = self.input.next().unwrap();
		let second = self.input.peek().cloned();
		match first {
			'<' => Token::Redirection(Redirection::Input),
			'>' => match second {
				Some('>') => {
					self.input.next();
					Token::Redirection(Redirection::Append)
				}
				Some('|') => Token::Redirection(Redirection::HereDoc),
				_ => Token::Redirection(Redirection::Output),
			},
			_ => Token::EndOfInput, // Fallback case
		}
	}
}

// Unit tests for the Lexer
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_simple_commands() {
		let mut lexer = Lexer::new("ls -la | grep 'txt'");
		let tokens = lexer.tokenize().unwrap();
		assert_eq!(
			tokens,
			vec![
				Token::Word("ls".to_string()),
				Token::Word("-la".to_string()),
				Token::Pipe,
				Token::Word("grep".to_string()),
				Token::Word("txt".to_string()),
				Token::EndOfInput
			]
		);
	}

	#[test]
	fn test_redirections() {
		let mut lexer = Lexer::new("echo \"Hello World\" > output.txt");
		let tokens = lexer.tokenize().unwrap();
		assert_eq!(
			tokens,
			vec![
				Token::Word("echo".to_string()),
				Token::Word("Hello World".to_string()),
				Token::Redirection(Redirection::Output),
				Token::Word("output.txt".to_string()),
				Token::EndOfInput
			]
		);
	}

	#[test]
	fn test_append_redirection() {
		let mut lexer = Lexer::new("echo 'Append this' >> file.txt");
		let tokens = lexer.tokenize().unwrap();
		assert_eq!(
			tokens,
			vec![
				Token::Word("echo".to_string()),
				Token::Word("Append this".to_string()),
				Token::Redirection(Redirection::Append),
				Token::Word("file.txt".to_string()),
				Token::EndOfInput
			]
		);
	}

	#[test]
	fn test_doubled_pipes() {
		let mut lexer = Lexer::new("ls || nah");
		let tokens = lexer.tokenize();
		assert!(tokens.is_ok());
	}

	#[test]
	fn test_doubled_pipes_space() {
		let mut lexer = Lexer::new("ls | | nah");
		let tokens = lexer.tokenize();
		assert_eq!(tokens, Err(LexerError::UnexpectedToken("|")));
	}
}
