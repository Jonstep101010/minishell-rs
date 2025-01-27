use crate::prelude::*;

fn check_exit_code(command: Vec<CString>) -> Result<i32, i32> {
	if command[1].is_empty() {
		eprint_msh!("exit: numeric argument required");
		Err(2)
	} else if command.len() > 2 {
		eprint_msh!("exit: too many arguments");
		Err(1)
	} else {
		match command[1].to_owned().into_string().unwrap().parse::<i32>() {
			Ok(code) => Ok(code),
			Err(_) => Err(1),
		}
	}
}

pub fn builtin_exit(shell_env: &mut Env, command: Vec<CString>) -> i32 {
	if command.len() > 1 {
		let exit_code = match check_exit_code(command) {
			Err(code) => return code,
			Ok(code) => code,
		};
		eprintln!("exit");
		std::process::exit(exit_code as u8 as i32);
	}
	eprintln!("exit");
	std::process::exit(shell_env.get_status());
}
