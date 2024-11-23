#![allow(
	mutable_transmutes,
	non_camel_case_types,
	non_snake_case,
	non_upper_case_globals,
	unused_mut,
	clippy::missing_safety_doc,
	clippy::upper_case_acronyms
)]

use msh::data::*;
use rustyline::{error::ReadlineError, DefaultEditor};
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut shell = Shell::default();
	dbg!(shell.clone());
	let mut rl = DefaultEditor::new()?;
	loop {
		let readline = rl.readline("core-rs>> ");
		match readline {
			Ok(line) => {
				let _ = rl.add_history_entry(line.as_str());
				shell.process_line(&line);
			}
			Err(ReadlineError::Interrupted) => {
				println!("CTRL-C");
				break;
			}
			Err(ReadlineError::Eof) => {
				continue;
			}
			Err(err) => {
				println!("Error: {:?}", err);
				break;
			}
		}
	}
	Ok(())
}
