use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Token {
	Word(String),
	Pipe,
	Redirection(Redirection),
	EndOfInput,
	LogicOperator(String),
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
	UnexpectedToken(String),
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

	pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
		let mut tokens = Vec::new();
		let mut last_was_pipe = false;
		let mut last_was_redirection = false;

		while let Some(&ch) = self.input.peek() {
			match ch {
				' ' | '\t' | '\n' => {
					self.input.next();
				}
				'|' => {
					self.input.next();
					if tokens.is_empty() {
						return Err(LexerError::UnexpectedToken("|".to_string()));
					}
					if let Some(&next_ch) = self.input.peek() {
						if next_ch == '|' {
							self.input.next();
							tokens.push(Token::LogicOperator("||".to_string()));
							last_was_pipe = false;
						} else if last_was_pipe {
							return Err(LexerError::UnexpectedToken("|".to_owned()));
						} else {
							tokens.push(Token::Pipe);
							last_was_pipe = true;
						}
					} else {
						return Err(LexerError::UnexpectedToken("|".to_owned()));
					}
				}
				'<' | '>' => {
					if last_was_redirection {
						let token_str = ch.to_string();
						return Err(LexerError::UnexpectedToken(token_str));
					}
					tokens.push(self.parse_redirection());
					last_was_pipe = false;
					last_was_redirection = true;
				}
				'\'' | '"' => {
					tokens.push(Token::Word(self.parse_quoted()));
					last_was_pipe = false;
					last_was_redirection = false;
				}
				'&' => {
					self.input.next();
					if let Some(&next_ch) = self.input.peek() {
						if next_ch == '&' {
							self.input.next();
							tokens.push(Token::LogicOperator("&&".to_string()));
							last_was_pipe = false;
							last_was_redirection = false;
						} else {
							return Err(LexerError::UnexpectedToken("&".to_owned()));
						}
					} else {
						return Err(LexerError::UnexpectedToken("&".to_owned()));
					}
				}
				_ => {
					tokens.push(Token::Word(self.parse_word()));
					last_was_pipe = false;
					last_was_redirection = false;
				}
			}
		}

		tokens.push(Token::EndOfInput);
		self.review_tokens(&tokens)?;
		Ok(tokens)
	}

	fn review_tokens(&self, tokens: &[Token]) -> Result<(), LexerError> {
		let mut last_was_redirection = false;
		let mut last_redirection = "";
		for token in tokens {
			match token {
				Token::Redirection(redir) => {
					if last_was_redirection {
						return Err(LexerError::UnexpectedToken(last_redirection.to_string()));
					}
					last_was_redirection = true;
					last_redirection = match redir {
						Redirection::Input => "<",
						Redirection::Output => ">",
						Redirection::Append => ">>",
						Redirection::HereDoc => "<<",
					};
				}
				Token::EndOfInput => {
					if last_was_redirection {
						return Err(LexerError::UnexpectedToken(last_redirection.to_string()));
					}
				}
				_ => {
					last_was_redirection = false;
				}
			}
		}
		Ok(())
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
			'<' => match second {
				Some('<') => {
					self.input.next();
					Token::Redirection(Redirection::HereDoc)
				}
				_ => Token::Redirection(Redirection::Input),
			},
			'>' => match second {
				Some('>') => {
					self.input.next();
					Token::Redirection(Redirection::Append)
				}
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
	fn test_here_doc_redirection() {
		let mut lexer = Lexer::new("cat << EOF");
		let tokens = lexer.tokenize().unwrap();
		assert_eq!(
			tokens,
			vec![
				Token::Word("cat".to_string()),
				Token::Redirection(Redirection::HereDoc),
				Token::Word("EOF".to_string()),
				Token::EndOfInput
			]
		);
	}

	#[test]
	fn test_input_redirection() {
		let mut lexer = Lexer::new("sort < input.txt");
		let tokens = lexer.tokenize().unwrap();
		assert_eq!(
			tokens,
			vec![
				Token::Word("sort".to_string()),
				Token::Redirection(Redirection::Input),
				Token::Word("input.txt".to_string()),
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
		assert_eq!(tokens, Err(LexerError::UnexpectedToken("|".to_string())));
	}

	#[test]
	fn test_logic_operators() {
		let mut lexer = Lexer::new("true && false || echo 'done'");
		let tokens = lexer.tokenize().unwrap();
		assert_eq!(
			tokens,
			vec![
				Token::Word("true".to_string()),
				Token::LogicOperator("&&".to_string()),
				Token::Word("false".to_string()),
				Token::LogicOperator("||".to_string()),
				Token::Word("echo".to_string()),
				Token::Word("done".to_string()),
				Token::EndOfInput
			]
		);
	}

	#[test]
	fn test_trailing_pipe_error() {
		let mut lexer = Lexer::new("echo 'Hello' |");
		let tokens = lexer.tokenize();
		assert_eq!(tokens, Err(LexerError::UnexpectedToken("|".to_string())));
	}

	#[test]
	fn test_starting_pipe_error() {
		let mut lexer = Lexer::new("| echo 'Hello'");
		let tokens = lexer.tokenize();
		assert_eq!(tokens, Err(LexerError::UnexpectedToken("|".to_string())));
	}

	#[test]
	fn test_invalid_consecutive_redirections() {
		let mut lexer = Lexer::new("< <");
		let tokens = lexer.tokenize();
		assert_eq!(tokens, Err(LexerError::UnexpectedToken("<".to_string())));
	}

	#[test]
	fn test_redirection_at_end_of_input() {
		let mut lexer = Lexer::new("echo 'Hello' >");
		let tokens = lexer.tokenize();
		assert_eq!(tokens, Err(LexerError::UnexpectedToken(">".to_string())));
	}

	#[test]
	fn test_valid_single_redirection() {
		let mut lexer = Lexer::new("< input.txt");
		let tokens = lexer.tokenize().unwrap();
		assert_eq!(
			tokens,
			vec![
				Token::Redirection(Redirection::Input),
				Token::Word("input.txt".to_string()),
				Token::EndOfInput
			]
		);
	}
}
