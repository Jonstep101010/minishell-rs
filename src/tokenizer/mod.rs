mod parse_quotes;
mod redirection_utils;
mod split_non_quoted;

use parse_quotes::rs_do_quote_bs;
use split_non_quoted::split_non_quoted;

use crate::prelude::*;

impl t_shell {
	///
	/// sets up pipes and their commands/arguments, including redirections
	pub(super) fn tokenize(&mut self, trimmed_line: &str) -> Option<()> {
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
