use crate::environment;
pub(crate) use crate::lexer::check as lexical_checks;

#[derive(Clone)]
pub struct t_shell {
	pub(crate) env: environment::Env,
	pub token_len: Option<usize>,
	pub token_vec: Vec<t_token>,
}

impl t_shell {
	pub fn new() -> Self {
		Self {
			env: environment::Env::new(),
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
pub struct t_token {
	pub cmd_args_vec: Vec<t_arg>,
	pub has_redir: bool,
	pub cmd_name: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct t_arg {
	pub elem_str: String,
	pub type_0: e_arg,          // wrapped enum attribute
	pub redir: Option<e_redir>, // enum wrapping string
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
pub enum e_arg {
	STRING = 0,
	REDIR = 1,
	REDIR_REMOVED = 2,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum e_redir {
	INPUT_REDIR = 1,
	OUTPUT_REDIR = 2,
	APPEND = 3,
	HEREDOC = 4,
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
