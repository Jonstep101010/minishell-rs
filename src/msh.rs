pub use crate::lexer::check as lexical_checks;
use crate::{environment, execution};

impl t_token {
	pub fn new(split_non_quoted: String) -> Self {
		Self {
			cmd_args: std::ptr::null_mut::<t_arg>(),
			has_redir: false,
			split_pipes: std::ptr::null_mut::<libc::c_char>(),
			tmp_arr: std::ptr::null_mut::<*mut libc::c_char>(),
			bin: std::ffi::CString::new("").unwrap(),
			cmd_func: Some(
				execution::exec_bin::exec_bin
					as unsafe fn(*mut t_shell, *mut t_token) -> libc::c_int,
			),
			split_non_quoted,
		}
	}
}

#[derive(Clone)]
#[repr(C)]
pub struct t_shell {
	pub exit_status: u8, // u8
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
	pub cmd_args: *mut t_arg,            // Vec<t_arg>
	pub has_redir: bool,                 // replace with Option<type>
	pub split_pipes: *mut libc::c_char,  // String
	pub tmp_arr: *mut *mut libc::c_char, // Vec<String>
	pub bin: std::ffi::CString,          // String
	pub cmd_func: Option<unsafe fn(*mut t_shell, *mut t_token) -> libc::c_int>, // fn
	pub split_non_quoted: String,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct t_arg {
	pub elem: *mut libc::c_char, // String
	pub type_0: e_arg,           // wrapped enum attribute
	pub redir: e_redir,          // enum wrapping string
}
#[derive(Copy, Clone)]
#[repr(C)]
pub enum e_arg {
	STRING = 0,
	REDIR = 1,
	REDIR_REMOVED = 2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub enum e_redir {
	NO_REDIR = 0,
	INPUT_REDIR = 1,
	OUTPUT_REDIR = 2,
	APPEND = 3,
	HEREDOC = 4,
}

pub type t_cmd_func_builtin =
	Option<unsafe fn(*mut crate::msh::t_shell, *mut t_token) -> libc::c_int>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct s_func {
	pub name: *mut libc::c_char,
	pub cmd: t_cmd_func_builtin,
}
