pub(crate) use crate::{environment::Env, eprint_msh};
pub(crate) use std::ffi::CString;

///
/// Store runtime data, allow access to parsed data
#[derive(Clone)]
pub(crate) struct ShellState {
	pub env: Env,
	pub token_len: Option<usize>,
	pub token_vec: Vec<CommandToken>,
}
impl ShellState {
	pub fn new() -> Self {
		Self {
			env: Env::new(),
			token_len: None,
			token_vec: vec![],
		}
	}
	///
	/// restores the `token_len` to the default value without input (new entry)
	pub fn restore(&mut self) {
		self.token_len = None;
	}
}

impl Default for ShellState {
	fn default() -> Self {
		Self::new()
	}
}

///
/// Store single Command ([`crate::msh::t_arg`]) separated by pipes in [`crate::msh::ShellState`]
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct CommandToken {
	pub cmd_args_vec: Vec<CommandArg>,
	pub has_redir: bool,
	pub cmd_name: Vec<u8>,
}

///
/// store single space-delimited argument
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct CommandArg {
	pub elem_str: String,
	pub type_0: Option<ArgType>,
	pub redir: Option<RedirType>,
}

impl CommandArg {
	pub fn new(elem_str_expanded: String) -> Self {
		Self {
			elem_str: elem_str_expanded,
			type_0: None,
			redir: None,
		}
	}
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum ArgType {
	Redir,
	RedirRemoved,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum RedirType {
	InputRedir,
	OutputRedir,
	Append,
	HereDoc,
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
