use crate::prelude::*;

fn declare_x(env: &Env) {
	for (key, value) in env.iter() {
		println!("declare -x {}={}", key, value);
	}
}

pub fn builtin_export(shell_env: &mut Env, command: Vec<CString>) -> i32 {
	if command.len() == 1 {
		declare_x(shell_env);
		return 0;
	}
	let mut i = 1;
	while i < command.len() {
		if !crate::environment::check_key::check_valid_key(command[i].as_bytes_with_nul()) {
			eprint_msh!(
				"export: `{}': not a valid identifier",
				command[i].to_str().unwrap()
			);
			return 1;
		}
		if command[i].as_bytes().contains(&b'=') {
			let kv = command[i].to_str().unwrap();
			let (key, value) = kv.split_once('=').unwrap();
			shell_env.export(key, value.to_string())
		}
		i += 1;
	}
	if i > 1 { 0 } else { 1 }
}
