use crate::{
	environment::{Env, expander::expander},
	parser::{interpret_quotes::rs_do_quote_bs, split_outside_quotes::split_non_quoted},
	prelude::*,
};

#[allow(unused_mut)]
unsafe fn init_token(mut size: usize) -> *mut t_token {
	let template: t_token = {
		t_token {
			has_redir: false,
			cmd_name: vec![],
			split_non_quoted: String::new(),
			cmd_args_vec: vec![],
		}
	};
	let mut token: *mut t_token =
		libc::calloc(size + 1, ::core::mem::size_of::<t_token>()) as *mut t_token;
	while !token.is_null() && {
		let fresh1 = size;
		size = size.wrapping_sub(1);
		fresh1 != 0
	} {
		libc::memcpy(
			&mut *token.add(size) as *mut t_token as *mut libc::c_void,
			&template as *const t_token as *const libc::c_void,
			::core::mem::size_of::<t_token>(),
		);
	}
	std::mem::forget(template);
	token
}

fn setup_token(token: &mut t_token, pipe_split: String, env: &Env) {
	let token_args = split_non_quoted(&pipe_split, " \t\n\r\x0B\x0C");
	token.cmd_args_vec = token_args
		.iter()
		.map(|arg| t_arg {
			type_0: STRING,
			redir: None,
			elem_str: {
				if arg.contains('$') {
					expander(arg, env).unwrap()
				} else {
					arg.to_string()
				}
			},
		})
		.collect();
}

pub unsafe fn tokenize(shell: &mut t_shell, trimmed_line: &str) -> Option<()> {
	let split_pipes = split_non_quoted(trimmed_line, "|");
	assert!(!split_pipes.is_empty());
	if split_pipes.first().unwrap().is_empty() {
		return None;
	}
	shell.token_len = Some(split_pipes.len());
	shell.token = init_token(shell.token_len.unwrap());
	if shell.token.is_null() {
		panic!("alloc fail token");
	}
	let mut i = 0;
	while i < shell.token_len.unwrap() {
		debug_assert!(!(shell.token).add(i).is_null());
		setup_token(
			&mut *(shell.token).add(i),
			split_pipes[i].clone(),
			&shell.env,
		);
		super::redirection_utils::process_redirections((shell.token).add(i));
		let mut ii = 0;
		while ii < (*(shell.token).add(i)).cmd_args_vec.len()
			&& !(*(shell.token).add(i)).cmd_args_vec[ii].elem_str.is_empty()
		{
			if (*(shell.token).add(i)).cmd_args_vec[ii].type_0 != REDIR {
				break;
			}
			ii += 1;
		}
		// set name of command
		(*(shell.token).add(i)).cmd_name = (*(shell.token).add(i)).cmd_args_vec[ii]
			.elem_str
			.as_mut_vec()
			.to_owned();
		let mut quote = 0;
		for arg in (*(shell.token).add(i)).cmd_args_vec.iter_mut() {
			arg.elem_str = rs_do_quote_bs(arg.elem_str.as_bytes(), &mut quote)
		}
		i += 1;
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
