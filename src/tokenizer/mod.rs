mod parse_quotes;
mod redirection_utils;
mod split_non_quoted;

use parse_quotes::rs_do_quote_bs;
use split_non_quoted::split_non_quoted;

use crate::msh::{Env, e_arg::*, t_arg, t_shell, t_token};

impl t_shell {
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
			.map(|piped_token| t_token::new(std::mem::take(piped_token), &self.env))
			.collect();
		Some(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::e_redir::*;
	use rstest::{fixture, rstest};
	macro_rules! token {
		($cmd:expr, $has_redir:expr, $($arg:expr, $type:expr, $redir:expr),*) => {
			t_token {
				cmd_args_vec: vec![
					$(
						t_arg {
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
				"echo", STRING, None,
				"hello", STRING, None
			),
			token!("cat", true,
				"cat", STRING, None,
				"outfile", REDIR, Some(OUTPUT_REDIR)
			)
		],
		"echo hello | cat > outfile"
	)]
	// manual equivalent
	// #[case(
	// 	vec![
	// 		t_token {
	// 			cmd_args_vec: vec![
	// 				t_arg {
	// 					elem_str: "echo".to_string(),
	// 					type_0: STRING,
	// 					redir: None,
	// 				},
	// 				t_arg {
	// 					elem_str: "hello".to_string(),
	// 					type_0: STRING,
	// 					redir: None,
	// 				},
	// 			],
	// 			has_redir: false,
	// 			cmd_name: vec![101, 99, 104, 111],
	// 		},
	// 		t_token {
	// 			cmd_args_vec: vec![
	// 				t_arg {
	// 					elem_str: "cat".to_string(),
	// 					type_0: STRING,
	// 					redir: None,
	// 				},
	// 				t_arg {
	// 					elem_str: "outfile".to_string(),
	// 					type_0: REDIR,
	// 					redir: Some(OUTPUT_REDIR),
	// 				},
	// 			],
	// 			has_redir: true,
	// 			cmd_name: vec![99, 97, 116],
	// 		},
	// 	],
	// 	"echo hello | cat > outfile"
	// )]
	#[case(
		vec![
			token!("ls", false,
				"ls", STRING, None,
				"-la", STRING, None
			),
			token!("grep", true,
				"grep", STRING, None,
				"test", STRING, None,
				"outfile", REDIR, Some(OUTPUT_REDIR)
			)
		],
		"ls -la | grep test > outfile"
	)]
	#[case(
		vec![
			token!("cat", true,
				"cat", STRING, None,
				"infile", REDIR, Some(INPUT_REDIR)
			),
			token!("wc", true,
				"wc", STRING, None,
				"-l", STRING, None,
				"result", REDIR, Some(OUTPUT_REDIR)
			)
		],
		"cat < infile | wc -l > result"
	)]
	#[fixture]
	fn test_tokenization(#[case] expected: Vec<t_token>, #[case] input: &str) {
		let trimmed_line = input.trim_ascii();
		let mut shell = t_shell::new();
		assert!(shell.tokenize(trimmed_line).is_some());
		assert_eq!(expected, shell.token_vec);
	}
}

impl t_token {
	/// for a split pipe, sets up the corresponding token (split whitespace, parse redirections, expand)
	pub fn new(pipe_split: String, shell_env: &Env) -> Self {
		let mut token = t_token {
			cmd_args_vec: split_non_quoted(&pipe_split, " \t\n\r\x0B\x0C")
				.iter_mut()
				.map(|arg| {
					shell_env.expander(arg);
					t_arg::new(std::mem::take(arg))
				})
				.collect(),
			has_redir: false,
			cmd_name: vec![],
		};
		token.process_redirections();
		let mut ii = 0;
		while ii < token.cmd_args_vec.len() && !token.cmd_args_vec[ii].elem_str.is_empty() {
			if token.cmd_args_vec[ii].type_0 != REDIR {
				break;
			}
			ii += 1;
		}
		// set name of command
		token.cmd_name = token.cmd_args_vec[ii]
			.elem_str
			.clone()
			.into_bytes()
			.to_owned();
		let mut quote = 0;
		for arg in token.cmd_args_vec.iter_mut() {
			arg.elem_str = rs_do_quote_bs(arg.elem_str.as_bytes(), &mut quote)
		}
		token
	}
}
