use crate::environment;
pub use crate::lexer::check as lexical_checks;

impl t_token {
	pub fn new(split_non_quoted: String) -> Self {
		Self {
			cmd_args_vec: vec![],
			has_redir: false,
			bin: std::ffi::CString::new("").unwrap(),
			cmd_func: None,
			split_non_quoted,
		}
	}
}

#[derive(Clone)]
#[repr(C)]
pub struct t_shell {
	pub exit_status: i32,
	pub(crate) env: environment::Env,
	pub token: *mut t_token, // Vec<t_token>
	pub token_len: Option<usize>,
	pub token_vec: Vec<t_token>,
}

impl t_shell {
	pub fn new() -> Self {
		Self {
			exit_status: 0,
			env: environment::Env::new(),
			token: std::ptr::null_mut(),
			token_len: None,
			token_vec: vec![],
		}
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
#[repr(C)]
pub struct t_token {
	pub cmd_args_vec: Vec<t_arg>,
	pub has_redir: bool,
	pub bin: std::ffi::CString, // String
	pub cmd_func: Option<unsafe fn(&mut t_shell, *mut t_token) -> i32>, // fn
	pub split_non_quoted: String,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct t_arg {
	pub elem: *mut libc::c_char, // String
	pub type_0: e_arg,           // wrapped enum attribute
	pub redir: Option<e_redir>,  // enum wrapping string
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub enum e_arg {
	STRING = 0,
	REDIR = 1,
	REDIR_REMOVED = 2,
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub enum e_redir {
	INPUT_REDIR = 1,
	OUTPUT_REDIR = 2,
	APPEND = 3,
	HEREDOC = 4,
}
