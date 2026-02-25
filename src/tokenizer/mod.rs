mod parse_quotes;
mod redirection_utils;
mod split_non_quoted;

use parse_quotes::rs_do_quote_bs;
use split_non_quoted::split_non_quoted;

use crate::msh::{ArgType::Redir, CommandArg, CommandToken as Token, Env, ShellState};

impl ShellState {
	/// Sets up pipes and their commands/arguments, including redirections
	///
	/// # Examples
	///
	/// Basic pipe operation:
	/// ```no_run
	/// # use minishell_rs::t_shell;
	/// let mut shell = t_shell::new();
	/// assert!(shell.tokenize("echo hello | cat").is_some());
	/// ```
	///
	/// Pipe with redirection:
	/// ```no_run
	/// # use minishell_rs::t_shell;
	/// let mut shell = t_shell::new();
	/// assert!(shell.tokenize("echo hello | cat > outfile").is_some());
	/// ```
	///
	/// Multiple pipes:
	/// ```no_run
	/// # use minishell_rs::t_shell;
	/// let mut shell = t_shell::new();
	/// assert!(shell.tokenize("ls -l | grep test | wc -l").is_some());
	/// ```
	///
	/// Empty input handling:
	/// ```no_run
	/// # use minishell_rs::t_shell;
	/// let mut shell = t_shell::new();
	/// assert!(shell.tokenize("").is_none());
	/// assert!(shell.tokenize("|").is_none());
	/// ```
	pub fn tokenize(&mut self, trimmed_line: &str) -> Option<()> {
		let mut split_pipes = split_non_quoted(trimmed_line, "|");
		assert!(!split_pipes.is_empty());
		if split_pipes.first().unwrap().is_empty() {
			return None;
		}
		self.token_len = Some(split_pipes.len());
		self.token_vec = split_pipes
			.iter_mut()
			.map(|piped_token| Token::new(piped_token, &self.env))
			.collect();
		Some(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::msh::RedirType::*;
	use rstest::rstest;
	macro_rules! token {
		($cmd:expr, $has_redir:expr, $($arg:expr, $type:expr, $redir:expr),*) => {
			Token {
				cmd_args_vec: vec![
					$(
						CommandArg {
							elem_str: $arg.to_string(),
							type_0: $type,
							redir: $redir,
						},
					)*
				],
				has_redir: $has_redir,
				cmd_name: $cmd.as_bytes().to_vec(),
			}
		};
	}
	#[rstest]
	#[case(
		vec![
			token!("echo", false,
				"echo", None, None,
				"hello", None, None
			),
			token!("cat", true,
				"cat", None, None,
				"outfile", Some(Redir), Some(OutputRedir)
			)
		],
		"echo hello | cat > outfile"
	)]
	#[case(
		vec![
			token!("ls", false,
				"ls", None, None,
				"-la", None, None
			),
			token!("grep", true,
				"grep", None, None,
				"test", None, None,
				"outfile", Some(Redir), Some(OutputRedir)
			)
		],
		"ls -la | grep test > outfile"
	)]
	#[case(
		vec![
			token!("cat", true,
				"cat", None, None,
				"infile", Some(Redir), Some(InputRedir)
			),
			token!("wc", true,
				"wc", None, None,
				"-l", None, None,
				"result", Some(Redir), Some(OutputRedir)
			)
		],
		"cat < infile | wc -l > result"
	)]
	#[case(
		vec![
			token!("", true,
				"infile", Some(Redir), Some(InputRedir)
			)
		],
		"< infile"
	)]
	fn test_tokenization(#[case] expected: Vec<Token>, #[case] input: &str) {
		let trimmed_line = input.trim_ascii();
		let mut shell = ShellState::new();
		assert!(shell.tokenize(trimmed_line).is_some());
		assert_eq!(expected, shell.token_vec);
	}
}

impl Token {
	/// for a split pipe, sets up the corresponding token (split whitespace, parse redirections, expand)
	pub fn new(pipe_split: &str, shell_env: &Env) -> Self {
		let mut token = Token {
			cmd_args_vec: split_non_quoted(pipe_split, " \t\n\r\x0B\x0C")
				.iter_mut()
				.map(|arg| {
					shell_env.expander(arg);
					CommandArg::new(std::mem::take(arg))
				})
				.collect(),
			has_redir: false,
			cmd_name: vec![],
		};
		token.process_redirections();
		let mut ii = 0;
		while ii < token.cmd_args_vec.len() && !token.cmd_args_vec[ii].elem_str.is_empty() {
			if token.cmd_args_vec[ii].type_0 != Some(Redir) {
				break;
			}
			ii += 1;
		}
		if ii < token.cmd_args_vec.len() {
			token.cmd_name = token.cmd_args_vec[ii].elem_str.clone().into_bytes();
		}
		let mut quote = 0;
		for arg in &mut token.cmd_args_vec {
			arg.elem_str = rs_do_quote_bs(arg.elem_str.as_bytes(), &mut quote);
		}
		token
	}
}
