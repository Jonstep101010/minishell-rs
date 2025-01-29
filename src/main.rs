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

// # Safety should only be active in very specific test scenarios
// extern crate libft_rs;
// extern crate libutils_rs;

mod prelude;
pub mod tokenizer;

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

pub mod utils {
	pub mod error;
	pub mod rust_readline;
} // mod utils

// handle signals? @note removed - non-functional
pub mod msh;
use prelude::*;

pub fn main() {
	let mut shell = t_shell::new();
	// check signals
	loop {
		if let Some(readline_line) = crate::utils::rust_readline::str_readline("minishell> ") {
			// b" \t\n\r\x0B\x0C\0"
			let trimmed_line = readline_line.trim_ascii();
			if trimmed_line.is_empty() {
				continue;
			}
			crate::utils::rust_readline::str_add_history(trimmed_line);
			if let Err(status) = msh::lexical_checks(trimmed_line) {
				shell.env.set_status(status);
				continue;
			} else if tokenizer::parse(&mut shell, trimmed_line).is_none() {
				shell.token_len = None;
				continue;
			} else {
				dbg!(&shell.token_vec);
				execution::execute_commands(&mut shell);
			}
		} else {
			std::process::exit(shell.env.get_status())
		}
	}
}
