#![allow(non_camel_case_types, non_snake_case, clippy::upper_case_acronyms)]
#![feature(let_chains)]
#![feature(extern_types)]

extern crate libc;

// to spend more time ;\
// handle signals? removed - non-functional
// use rustyline instead of readline
// use process instead of fork
// use tempfile for heredoc (if possible!)

mod environment;
mod execution;
mod lexer;

pub mod utils {
	pub mod error;
	pub mod rust_readline;
}

pub mod msh;
mod prelude;
mod tokenizer;
use prelude::*;

pub fn main() {
	let mut shell = t_shell::new();
	// check signals
	loop {
		if let Some(readline_line) = crate::utils::rust_readline::str_readline("minishell> ") {
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
				crate::execution::execute_commands(&mut shell);
			}
		} else {
			std::process::exit(shell.env.get_status())
		}
	}
}
