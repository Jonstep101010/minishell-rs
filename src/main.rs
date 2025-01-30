#![allow(non_camel_case_types, non_snake_case, clippy::upper_case_acronyms)]
#![feature(let_chains)]
#![feature(extern_types)]

extern crate libc;

// to spend more time ;\
// handle signals? removed - non-functional - partially working in rustyline
// use tempfile for heredoc (if possible!)

mod environment;
mod execution;
mod lexer;
mod msh;
mod tokenizer;
use msh::*;
use rustyline::{DefaultEditor, Result, error::ReadlineError};

pub fn main() -> Result<()> {
	let mut shell = t_shell::new();
	// check signals
	let mut rl = DefaultEditor::new()?;
	loop {
		let readline = rl.readline("minishell> ");
		match readline {
			Ok(line) => {
				let trimmed_line = line.trim_ascii();
				if trimmed_line.is_empty() {
					continue;
				}
				rl.add_history_entry(trimmed_line)?;
				if let Err(status) = lexer::check(trimmed_line) {
					shell.env.set_status(status);
					continue;
				} else if shell.tokenize(trimmed_line).is_none() {
					continue;
				} else {
					// dbg!(&shell.token_vec);
					crate::execution::execute_commands(&mut shell);
				}
			}
			Err(ReadlineError::Interrupted) => {
				continue;
			}
			Err(ReadlineError::Eof) => {
				println!("CTRL-D");
				break;
			}
			Err(err) => {
				println!("Error: {:?}", err);
				break;
			}
		}
	}
	if shell.env.get_status() == 0 {
		Ok(())
	} else {
		Err(ReadlineError::Io(std::io::Error::new(
			std::io::ErrorKind::Other,
			"Non-zero exit status",
		)))
	}
}
