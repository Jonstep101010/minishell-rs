#![allow(
	mutable_transmutes,
	non_camel_case_types,
	non_snake_case,
	non_upper_case_globals,
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
pub mod tokenizer {
	pub mod build_command;
	pub mod build_tokens;
	pub mod destroy_tokens;
	pub mod redirection_utils;
} // mod tokenizer
pub mod utils {
	pub mod bool_array;
	pub mod error;
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

#[allow(unused_mut)]
unsafe fn main_0() -> i32 {
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
			if let Err(status) = msh::lexical_checks(trimmed_line) {
				shell.env.set_status(status);
				continue;
			} else {
				tokenizer::build_tokens::tokenize(&mut shell, trimmed_line);
				if (shell.token).is_null() {
					tokenizer::destroy_tokens::destroy_all_tokens(&mut shell);
					continue;
				}
				if ((*shell.token).cmd_args_vec).is_empty() {
					tokenizer::destroy_tokens::destroy_all_tokens(&mut shell);
					continue;
				}
			}
			self::t_shell::create_tokens(&mut shell, trimmed_line);
			dbg!(&shell.token_vec);
			if !(shell.token).is_null() {
				execution::execute_commands(&mut shell);
			}
		} else {
			std::process::exit(shell.env.get_status())
		}
	}
}
pub fn main() {
	unsafe { ::std::process::exit(main_0()) }
}
