#![allow(
	mutable_transmutes,
	non_camel_case_types,
	non_snake_case,
	non_upper_case_globals,
	unused_mut,
	clippy::missing_safety_doc,
	clippy::upper_case_acronyms
)]
#![feature(let_chains)]
#![feature(extern_types)]
#![allow(unsafe_op_in_unsafe_fn)]

extern crate libc;
extern crate libft_rs;
extern crate libutils_rs;

unsafe extern "C" {
	fn __errno_location() -> *mut libc::c_int;
}

mod prelude;
use prelude::*;
use utils::rust_readline::{str_add_history, str_readline};

pub mod builtins {
	pub mod cd;
	pub mod echo;
	pub mod env;
	pub mod exit;
	pub mod export;
	pub mod pwd;
	pub mod unset;
} // mod builtins
pub mod environment; // mod environment
pub mod execution; // mod execution
pub mod lexer {
	use crate::{
		size_t, t_shell,
		tokenizer::{build_tokens::tokenize, destroy_tokens::destroy_all_tokens},
		utils::get_input::get_input,
	};
	use ::libc::{self, free};

	#[derive(Copy, Clone)]
	#[repr(C)]
	pub struct t_lexer {
		pub singlequotes: libc::c_int,
		pub doublequotes: libc::c_int,
		pub open_curly_brackets: libc::c_int,
		pub close_curly_brackets: libc::c_int,
		pub open_square_brackets: libc::c_int,
		pub close_square_brackets: libc::c_int,
		pub open_parentheses: libc::c_int,
		pub close_parentheses: libc::c_int,
		pub redir_greater: libc::c_int,
		pub redir_smaller: libc::c_int,
		pub pipes: libc::c_int,
		pub ignore: *mut bool,
		pub len: size_t,
		pub lexer: libc::c_int,
		pub result: bool,
	}
	use checks_basic::lexer_checks_basic;
	#[unsafe(no_mangle)]
	pub unsafe extern "C" fn run(
		mut shell: *mut t_shell,
		mut trimmed_line: *const libc::c_char,
	) -> libc::c_int {
		if *trimmed_line == 0 {
			get_input(std::ptr::null_mut::<libc::c_char>());
			return 0 as libc::c_int;
		}
		let mut lex = lexer_checks_basic(trimmed_line);
		if !(*lex).result {
			(*shell).exit_status = (*lex).lexer as u8;
			get_input(std::ptr::null_mut::<libc::c_char>());
			free(lex as *mut libc::c_void);
			return 1 as libc::c_int;
		}
		free(lex as *mut libc::c_void);
		(*shell).token = tokenize(shell, trimmed_line) as *mut crate::t_token;
		get_input(std::ptr::null_mut::<libc::c_char>());
		if ((*shell).token).is_null() {
			return -(1 as libc::c_int);
		}
		if ((*(*shell).token).cmd_args).is_null() {
			destroy_all_tokens(shell);
			return -(1 as libc::c_int);
		}
		0 as libc::c_int
	}

	mod check_pipes;
	mod checks_basic;
	mod lexer_support;
} // mod lexer
pub mod parser {
	pub mod interpret_quotes;
	pub mod split_outside_quotes;
} // mod parser
pub mod signals {
	pub mod handlers;
} // mod signals
pub mod tokenizer {
	pub mod build_command;
	pub mod build_tokens;
	pub mod destroy_tokens;
	pub mod redirection_utils;
	pub mod token_utils;
} // mod tokenizer
pub mod utils {
	pub mod bool_array;
	pub mod error;
	pub mod exit_free;
	pub mod get_input;
	pub mod init_shell;
	pub mod interop;
	pub mod rust_readline;
} // mod utils

#[derive(Copy, Clone)]
#[repr(C)]
pub struct termios {
	pub c_iflag: tcflag_t,
	pub c_oflag: tcflag_t,
	pub c_cflag: tcflag_t,
	pub c_lflag: tcflag_t,
	pub c_line: cc_t,
	pub c_cc: [cc_t; 32],
	pub c_ispeed: speed_t,
	pub c_ospeed: speed_t,
}
#[derive(Clone)]
#[repr(C)]
pub struct t_token {
	pub cmd_args: *mut t_arg, // Vec<t_arg>
	pub has_redir: bool,
	pub split_pipes: *mut libc::c_char,  // String
	pub tmp_arr: *mut *mut libc::c_char, // Vec<String>
	pub bin: std::ffi::CString,          // String
	pub cmd_func: Option<unsafe extern "C" fn(*mut t_shell, *mut t_token) -> libc::c_int>, // fn
}

#[derive(Clone)]
#[repr(C)]
pub struct t_shell {
	pub exit_status: uint8_t, // u8
	env: environment::Env,
	pub token: *mut t_token, // Vec<t_token>
	pub token_len: size_t,
}

impl t_shell {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct t_arg {
	pub elem: *mut libc::c_char, // String
	pub type_0: e_arg,           // wrapped enum attribute
	pub redir: e_redir,          // enum wrapping string
}

unsafe fn main_0() -> libc::c_int {
	let mut shell: *mut t_shell = utils::init_shell::init_shell();
	if shell.is_null() {
		return 1 as libc::c_int;
	}
	// check signals
	loop {
		if let Some(readline_line) = str_readline("minishell> ") {
			// b" \t\n\r\x0B\x0C\0"
			let mut trimmed_line = readline_line.trim_ascii();
			if trimmed_line.is_empty() {
				continue;
			}
			str_add_history(trimmed_line);
			if crate::lexer::run(
				shell,
				std::ffi::CString::new(trimmed_line).unwrap().as_ptr(),
			) != 0 as libc::c_int
			{
				continue;
			}
			if !((*shell).token).is_null() {
				execution::execute_commands(shell, (*shell).token);
			}
		} else {
			builtins::exit::builtin_exit(shell, std::ptr::null_mut::<t_token>());
		}
	}
}
pub fn main() {
	unsafe { ::std::process::exit(main_0() as i32) }
}
