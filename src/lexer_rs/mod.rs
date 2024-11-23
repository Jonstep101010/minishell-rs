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

impl<'a> Lexer<'a> {
	pub fn new(input: &'a str) -> Self {
		Lexer {
			input: input.chars().peekable(),
		}
	}

	pub fn tokenize(&mut self) -> Vec<Token> {
		let mut tokens = Vec::new();

		while let Some(&ch) = self.input.peek() {
			match ch {
				' ' | '\t' | '\n' => {
					self.input.next();
				}
				'|' => {
					self.input.next();
					tokens.push(Token::Pipe);
				}
				'<' | '>' => {
					tokens.push(self.parse_redirection());
				}
				'\'' | '"' => {
					tokens.push(Token::Word(self.parse_quoted()));
				}
				_ => {
					tokens.push(Token::Word(self.parse_word()));
				}
			}
		}

		tokens.push(Token::EndOfInput);
		tokens
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
		let tokens = lexer.tokenize();
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
		let tokens = lexer.tokenize();
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
		let tokens = lexer.tokenize();
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
}
