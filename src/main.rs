#![allow(
	dead_code,
	mutable_transmutes,
	non_camel_case_types,
	non_snake_case,
	non_upper_case_globals,
	unused_assignments,
	unused_mut,
	clippy::missing_safety_doc
)]
#![feature(c_variadic)]
#![feature(extern_types)]

extern crate libc;
extern crate libft_rs;
extern crate libftprintf_rs;
extern crate libgnl_rs;
extern crate libutils_rs;

pub mod builtins {
	pub mod builtin_cd;
	pub mod builtin_echo;
	pub mod builtin_env;
	pub mod builtin_exit;
	pub mod builtin_export;
	pub mod builtin_pwd;
	pub mod builtin_unset;
} // mod builtins
pub mod environment {
	pub mod check_key;
	pub mod expander;
	pub mod export_env;
	pub mod get_env;
	pub mod get_index;
} // mod environment
pub mod execution {
	mod bin_path;
	mod exec_bin;
	pub mod execute_commands;
	mod execute_pipes;
	mod heredoc;
	mod redirections;
} // mod execution
pub mod lexer {
	use crate::environment::export_env::update_exit_status;
	use crate::tokenizer::{build_tokens::tokenize, destroy_tokens::destroy_all_tokens};
	use crate::{t_shell, utils::get_input::get_input};
	use ::libc::{self, free};

	pub type size_t = libc::c_ulong;
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
	pub type speed_t = libc::c_uint;
	pub type cc_t = libc::c_uchar;
	pub type tcflag_t = libc::c_uint;
	pub type uint8_t = __uint8_t;
	pub type __uint8_t = libc::c_uchar;
	use crate::t_token;
	pub type e_redir = libc::c_uint;
	pub const HEREDOC: e_redir = 4;
	pub const APPEND: e_redir = 3;
	pub const OUTPUT_REDIR: e_redir = 2;
	pub const INPUT_REDIR: e_redir = 1;
	pub const NO_REDIR: e_redir = 0;
	pub type e_arg = libc::c_uint;
	pub const REDIR_REMOVED: e_arg = 2;
	pub const REDIR: e_arg = 1;
	pub const STRING: e_arg = 0;
	#[no_mangle]
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
			update_exit_status(shell, (*lex).lexer);
			get_input(std::ptr::null_mut::<libc::c_char>());
			free(lex as *mut libc::c_void);
			return 1 as libc::c_int;
		}
		free(lex as *mut libc::c_void);
		(*shell).token = tokenize(shell, trimmed_line) as *mut t_token;
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
	pub mod arr_utils;
	pub mod bool_array;
	pub mod error;
	pub mod exit_free;
	pub mod get_input;
	pub mod init_shell;
	pub mod str_equal;
} // mod utils

extern "C" {
	fn readline(_: *const libc::c_char) -> *mut libc::c_char;
	fn add_history(_: *const libc::c_char);
	fn check_signals(p_termios: *mut termios);
	// fn lexer(shell: *mut t_shell, trimmed_line: *const libc::c_char) -> libc::c_int;
	// fn execute_commands(shell: *mut t_shell, token: *mut t_token);
	// fn init_shell(envp: *const *mut libc::c_char) -> *mut t_shell;
	// fn get_input(rl_prompt: *mut libc::c_char) -> *mut libc::c_char;
	// fn builtin_exit(shell: *mut t_shell, nullable: *mut t_token) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type cc_t = libc::c_uchar;
pub type speed_t = libc::c_uint;
pub type tcflag_t = libc::c_uint;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct t_token {
	pub cmd_args: *mut t_arg,
	pub has_redir: bool,
	pub split_pipes: *mut libc::c_char,
	pub tmp_arr: *mut *mut libc::c_char,
	pub bin: *mut libc::c_char,
	pub cmd_func: Option<unsafe extern "C" fn(*mut t_shell, *mut t_token) -> libc::c_int>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct t_shell {
	pub exit_status: uint8_t,
	pub env: *mut *mut libc::c_char,
	pub token: *mut t_token,
	pub token_len: size_t,
	pub p_termios: termios,
}
pub type uint8_t = __uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct t_arg {
	pub elem: *mut libc::c_char,
	pub type_0: e_arg,
	pub redir: e_redir,
}
pub type e_redir = libc::c_uint;
pub const HEREDOC: e_redir = 4;
pub const APPEND: e_redir = 3;
pub const OUTPUT_REDIR: e_redir = 2;
pub const INPUT_REDIR: e_redir = 1;
pub const NO_REDIR: e_redir = 0;
pub type e_arg = libc::c_uint;
pub const REDIR_REMOVED: e_arg = 2;
pub const REDIR: e_arg = 1;
pub const STRING: e_arg = 0;

pub type __pid_t = libc::c_int;
pub type pid_t = __pid_t;

unsafe fn main_0(
	mut _ac: libc::c_int,
	mut _av: *mut *mut libc::c_char,
	mut envp: *mut *mut libc::c_char,
) -> libc::c_int {
	let mut shell: *mut t_shell = std::ptr::null_mut::<t_shell>();
	shell = utils::init_shell::init_shell(envp);
	if shell.is_null() {
		return 1 as libc::c_int;
	}
	let mut trimmed_line: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
	let mut readline_line: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
	check_signals(&mut (*shell).p_termios);
	loop {
		readline_line = readline(b"minishell> \0" as *const u8 as *const libc::c_char);
		if readline_line.is_null() {
			builtins::builtin_exit::builtin_exit(shell, std::ptr::null_mut::<t_token>());
		}
		trimmed_line = utils::get_input::get_input(readline_line);
		if trimmed_line.is_null() {
			continue;
		}
		add_history(trimmed_line);
		if *trimmed_line == 0 || crate::lexer::run(shell, trimmed_line) != 0 as libc::c_int {
			continue;
		}
		if !((*shell).env).is_null() && !(*(*shell).env).is_null() && !((*shell).token).is_null() {
			execution::execute_commands::execute_commands(shell, (*shell).token);
		}
	}
}
pub fn main() {
	let mut args: Vec<*mut libc::c_char> = Vec::new();
	for arg in ::std::env::args() {
		args.push(
			(::std::ffi::CString::new(arg))
				.expect("Failed to convert argument into CString.")
				.into_raw(),
		);
	}
	args.push(::core::ptr::null_mut());
	let mut vars: Vec<*mut libc::c_char> = Vec::new();
	for (var_name, var_value) in ::std::env::vars() {
		let var: String = format!("{}={}", var_name, var_value);
		vars.push(
			(::std::ffi::CString::new(var))
				.expect("Failed to convert environment variable into CString.")
				.into_raw(),
		);
	}
	vars.push(::core::ptr::null_mut());
	unsafe {
		::std::process::exit(main_0(
			(args.len() - 1) as libc::c_int,
			args.as_mut_ptr(),
			vars.as_mut_ptr(),
		) as i32)
	}
}
