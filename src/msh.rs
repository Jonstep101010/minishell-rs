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
	pub fn export(&mut self, key: &str, value: String) {
		self.env.export(key, value);
	}
	pub fn unset(&mut self, key: &str) {
		self.env.unset(key);
	}
	pub fn get_var(&self, key: &str) -> Option<&String> {
		self.env.get(key)
	}
}

impl Default for t_shell {
	fn default() -> Self {
		Self::new()
	}
}

#[derive(Clone, Debug)]
pub struct t_token {
	pub cmd_args_vec: Vec<t_arg>,
	pub has_redir: bool,
	pub cmd_name: Vec<u8>,
	pub split_non_quoted: String,
}

#[derive(Clone, Debug)]
pub struct t_arg {
	pub elem_str: String,
	pub type_0: e_arg,          // wrapped enum attribute
	pub redir: Option<e_redir>, // enum wrapping string
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
