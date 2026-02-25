use crate::{Env, eprint_msh};
use std::ffi::CString;

fn check_exit_code(command: &[CString]) -> Result<i32, i32> {
	if command[1].is_empty() {
		eprint_msh!("exit: numeric argument required");
		Err(2)
	} else if command.len() > 2 {
		eprint_msh!("exit: too many arguments");
		Err(1)
	} else {
		match command[1].clone().into_string().unwrap().parse::<i32>() {
			Ok(code) => Ok(code),
			Err(_) => Err(1),
		}
	}
}

pub fn exit(shell_env: &mut Env, command: &[CString]) -> i32 {
	if command.len() > 1 {
		match check_exit_code(command) {
			Err(code) => return code,
			Ok(code) => {
				eprintln!("exit");
				std::process::exit(code.rem_euclid(256));
			}
		};
	}
	eprintln!("exit");
	std::process::exit(shell_env.get_status());
}
