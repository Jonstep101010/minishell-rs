pub(crate) use crate::{environment::Env, eprint_msh};
pub(crate) use std::ffi::CString;

#[derive(Clone)]
pub(crate) struct t_shell {
	pub env: Env,
	pub token_len: Option<usize>,
	pub token_vec: Vec<t_token>,
}

impl t_shell {
	pub fn new() -> Self {
		Self {
			env: Env::new(),
			token_len: None,
			token_vec: vec![],
		}
	}
	///
	/// restores the token_len to the default value without input (new entry)
	pub fn restore(&mut self) {
		self.token_len = None;
	}
}

impl Default for t_shell {
	fn default() -> Self {
		Self::new()
	}
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct t_token {
	pub cmd_args_vec: Vec<t_arg>,
	pub has_redir: bool,
	pub cmd_name: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct t_arg {
	pub elem_str: String,
	pub type_0: e_arg,
	pub redir: Option<e_redir>,
}

impl t_arg {
	pub fn new(elem_str_expanded: String) -> Self {
		Self {
			elem_str: elem_str_expanded,
			type_0: e_arg::STRING,
			redir: None,
		}
	}
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum e_arg {
	STRING,
	REDIR,
	REDIR_REMOVED,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum e_redir {
	INPUT_REDIR,
	OUTPUT_REDIR,
	APPEND,
	HEREDOC,
}

#[macro_export]
macro_rules! eprint_msh {
	($($arg:tt)*) => {
		{
			use std::io::Write;
			let stderr = std::io::stderr();
			let mut handle = stderr.lock();
			write!(handle, "minishell: ").unwrap();
			write!(handle, $($arg)*).unwrap();
			writeln!(handle).unwrap();
		}
	};
}
