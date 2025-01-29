use crate::{
	environment::{Env, expander::expander},
	parser::{interpret_quotes::rs_do_quote_bs, split_outside_quotes::split_non_quoted},
	prelude::*,
};

pub fn tokenize(shell: &mut t_shell, trimmed_line: &str) -> Option<()> {
	let mut split_pipes = split_non_quoted(trimmed_line, "|");
	assert!(!split_pipes.is_empty());
	if split_pipes.first().unwrap().is_empty() {
		return None;
	}
	shell.token_len = Some(split_pipes.len());
	// shell.token = init_token(shell.token_len.unwrap());
	shell.token_vec = split_pipes
		.iter_mut()
		.map(|piped_token| t_token {
			cmd_args_vec: vec![],
			has_redir: false,
			cmd_name: vec![],
			split_non_quoted: std::mem::take(piped_token),
		})
		.collect();
	for mut token in &mut shell.token_vec {
		// debug_assert!(!(shell.token).add(i).is_null());
		let token_args = split_non_quoted(&token.split_non_quoted, " \t\n\r\x0B\x0C");
		token.cmd_args_vec = token_args
			.iter()
			.map(|arg| t_arg {
				type_0: STRING,
				redir: None,
				elem_str: {
					if arg.contains('$') {
						expander(arg, &shell.env).unwrap()
					} else {
						arg.to_string()
					}
				},
			})
			.collect();
		super::redirection_utils::process_redirections(token);
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
	}
	Some(())
}

impl t_token {
	pub fn new(split_non_quoted: String) -> Self {
		Self {
			cmd_args_vec: vec![],
			has_redir: false,
			cmd_name: vec![],
			split_non_quoted,
		}
	}
}

// @todo testing before usage
impl t_shell {
	///
	/// future replacement for `get_tokens`
	pub fn create_piped_tokens(&mut self, trimmed_line: &str) {
		let split_pipes = crate::parser::split_outside_quotes::split_non_quoted(trimmed_line, "|");
		self.token_vec = split_pipes
			.iter()
			.map(|single_pipe| t_token::new(single_pipe.to_owned()))
			.collect();
		self.token_len = if self.token_vec.is_empty() {
			None
		} else {
			Some(self.token_vec.len())
		}
	}
}
