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
pub mod lexer; // mod lexer
pub mod parser {
	pub mod interpret_quotes;
	pub mod split_outside_quotes;
} // mod parser
// pub mod signals {
// 	pub mod handlers;
// } // mod signals
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
	pub mod interop;
	pub mod rust_readline;
} // mod utils

// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct termios {
// 	pub c_iflag: tcflag_t,
// 	pub c_oflag: tcflag_t,
// 	pub c_cflag: tcflag_t,
// 	pub c_lflag: tcflag_t,
// 	pub c_line: cc_t,
// 	pub c_cc: [cc_t; 32],
// 	pub c_ispeed: speed_t,
// 	pub c_ospeed: speed_t,
// }
pub mod msh;
pub use prelude::*;

unsafe fn main_0() -> libc::c_int {
	// let mut shell: *mut t_shell = utils::init_shell::init_shell();
	// if shell.is_null() {
	// 	return 1 as libc::c_int;
	// }
	let mut shell = t_shell::new();
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
				&mut shell,
				std::ffi::CString::new(trimmed_line).unwrap().as_ptr(),
			) != 0 as libc::c_int
			{
				continue;
			}
			self::t_shell::create_tokens(&mut shell, trimmed_line);
			dbg!(&shell.token_vec);
			if !(shell.token).is_null() {
				execution::execute_commands(&mut shell, shell.token);
			}
		} else {
			builtins::exit::builtin_exit(&mut shell, std::ptr::null_mut::<t_token>());
		}
	}
}
pub fn main() {
	unsafe { ::std::process::exit(main_0() as i32) }
}
